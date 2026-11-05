use crate::input::TogglePause;
use crate::loading::AppLoadingState;
use crate::ui::{FontType, SpawnMenu};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};
use bevy_enhanced_input::prelude::Start;
use bevy_fix_cursor_unlock_web::ForceUnlockCursor;

pub fn plugin(app: &mut App) {
    app.add_sub_state::<PauseState>()
        .add_systems(
            OnEnter(PauseState::Paused),
            (pause_time, release_cursor, spawn_pause_menu),
        )
        .add_systems(OnExit(PauseState::Paused), (resume_time, capture_cursor))
        .add_observer(toggle_pause)
        .add_observer(pause_on_cursor_unlock);
}

#[derive(SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[source(AppLoadingState = AppLoadingState::Loaded)]
pub enum PauseState {
    #[default]
    Paused,
    Running,
}

fn spawn_pause_menu(mut writer: MessageWriter<SpawnMenu>) {
    writer.write(SpawnMenu {
        text: "🜋 P🜁🜰S🝗D 🜋".to_uppercase(),
        font: FontType::Symbols,
        state: Some(PauseState::Paused),
    });
}

fn toggle_pause(
    _: On<Start<TogglePause>>,
    state: Option<Res<State<PauseState>>>,
    mut next: ResMut<NextState<PauseState>>,
) {
    // `PauseState` doesn't exist before loading finished.
    let Some(state) = state else {
        return;
    };
    match state.get() {
        PauseState::Running => next.set(PauseState::Paused),
        // do noting on web
        PauseState::Paused if cfg!(target_arch = "wasm32") => {}
        PauseState::Paused => next.set(PauseState::Running),
    }
}

/// Try to pause game when cursor is unlocked on the web.
fn pause_on_cursor_unlock(
    _: On<ForceUnlockCursor>,
    state: Option<Res<State<PauseState>>>,
    mut next: ResMut<NextState<PauseState>>,
    mut cursor: Single<&mut CursorOptions>,
) {
    set_cursor_captured(&mut cursor, false);
    if state.is_some_and(|state| *state.get() == PauseState::Running) {
        next.set(PauseState::Paused);
    }
}

fn pause_time(mut time: ResMut<Time<Virtual>>) {
    time.pause();
}

fn resume_time(mut time: ResMut<Time<Virtual>>) {
    time.unpause();
}

fn capture_cursor(mut cursor: Single<&mut CursorOptions>) {
    set_cursor_captured(&mut cursor, true);
}

fn release_cursor(mut cursor: Single<&mut CursorOptions>) {
    set_cursor_captured(&mut cursor, false);
}

fn set_cursor_captured(cursor: &mut CursorOptions, captured: bool) {
    cursor.grab_mode = if captured {
        CursorGrabMode::Locked
    } else {
        CursorGrabMode::None
    };
    cursor.visible = !captured;
}
