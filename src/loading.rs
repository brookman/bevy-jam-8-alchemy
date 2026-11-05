use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::ui::SpawnLoadingText;

pub fn plugin(app: &mut App) {
    let config = LoadingState::new(AppLoadingState::Loading)
        .continue_to_state(AppLoadingState::Loaded)
        .on_failure_continue_to_state(AppLoadingState::Failed);

    app.init_state::<AppLoadingState>()
        .add_loading_state(config)
        .add_systems(OnEnter(AppLoadingState::Loading), spawn_loading_text)
        .add_systems(OnEnter(AppLoadingState::Failed), spawn_failed_text);
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppLoadingState {
    #[default]
    Loading,
    Loaded,
    Failed,
}

fn spawn_loading_text(mut writer: MessageWriter<SpawnLoadingText>) {
    writer.write(SpawnLoadingText {
        text: "Loading...".to_string(),
    });
}

fn spawn_failed_text(mut writer: MessageWriter<SpawnLoadingText>) {
    writer.write(SpawnLoadingText {
        text: "Failed to load assets, see log".to_string(),
    });
}

pub trait LoadCollection {
    fn load_collection<A: AssetCollection>(self: &mut Self) -> &mut Self;
}

impl LoadCollection for App {
    fn load_collection<A: AssetCollection>(self: &mut Self) -> &mut Self {
        self.configure_loading_state(
            LoadingStateConfig::new(AppLoadingState::Loading).load_collection::<A>(),
        )
    }
}
