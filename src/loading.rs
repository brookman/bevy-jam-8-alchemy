use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::Running)
                .on_failure_continue_to_state(AppState::LoadFailed),
        )
        .add_systems(
            OnEnter(AppState::Loading),
            |commands: Commands, asset_server: Res<AssetServer>| {
                spawn_message(
                    commands,
                    asset_server,
                    "Loading...\n🜁🜂🜃🜄🜅🜆🜇🜈🜉🜊🜋🜌🜍🜎🜏",
                    AppState::Loading,
                );
            },
        )
        .add_systems(
            OnEnter(AppState::LoadFailed),
            |commands: Commands, asset_server: Res<AssetServer>| {
                spawn_message(
                    commands,
                    asset_server,
                    "Failed to load assets, see log",
                    AppState::LoadFailed,
                );
            },
        );
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Loading,
    Running,
    LoadFailed,
}

pub trait LoadCollection {
    fn load_collection<A: AssetCollection>(self: &mut Self) -> &mut Self;
}

impl LoadCollection for App {
    fn load_collection<A: AssetCollection>(self: &mut Self) -> &mut Self {
        self.configure_loading_state(
            LoadingStateConfig::new(AppState::Loading).load_collection::<A>(),
        )
    }
}

pub fn spawn_message<S: States>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    text: &'static str,
    state: S,
) {
    commands.spawn((
        Name::new(text),
        DespawnOnExit(state),
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Pickable::IGNORE,
        children![(
            Text::new(text.to_uppercase()),
            TextFont {
                font: asset_server
                    .load("fonts/NotoSansSymbols-VariableFont_wght.ttf")
                    .into(),
                font_size: FontSize::Px(48.0),
                ..default()
            },
            Pickable::IGNORE,
        )],
    ));
}
