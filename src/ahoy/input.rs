use bevy::app::{App, Update};
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::{Component, IntoScheduleConfigs, KeyCode, MouseButton, On, Single};
use bevy::window::{CursorGrabMode, CursorOptions};
use bevy_enhanced_input::prelude::*;
use bevy_fix_cursor_unlock_web::{FixPointerUnlockPlugin, ForceUnlockCursor};

pub fn plugin(app: &mut App) {
    app.add_plugins(FixPointerUnlockPlugin)
        .add_input_context::<PlayerInput>()
        .add_observer(unlock_cursor_web)
        .add_systems(
            Update,
            (
                capture_cursor.run_if(input_just_pressed(MouseButton::Left)),
                release_cursor.run_if(input_just_pressed(KeyCode::Escape)),
            ),
        );
}

#[derive(Component, Default, Clone)]
pub(crate) struct PlayerInput;

fn unlock_cursor_web(
    _unlock: On<ForceUnlockCursor>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    cursor_options.grab_mode = CursorGrabMode::None;
    cursor_options.visible = true;
}

fn capture_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.grab_mode = CursorGrabMode::Locked;
    cursor.visible = false;
}

fn release_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.visible = true;
    cursor.grab_mode = CursorGrabMode::None;
}
