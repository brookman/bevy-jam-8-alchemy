use crate::{
    loading::{AppLoadingState, LoadCollection},
    pause::PauseState,
};
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

const DEFAULT_TEXT_SIZE: f32 = 48.0;

pub fn plugin(app: &mut App) {
    app.load_collection::<FontsAsssets>()
        .add_message::<SpawnLoadingText>()
        .add_message::<SpawnText>()
        .add_systems(
            Update,
            (
                on_spawn_loading_text,
                on_spawn_text.run_if(resource_exists::<FontsAsssets>),
            ),
        );
}

#[derive(AssetCollection, Resource)]
struct FontsAsssets {
    #[asset(path = "fonts/NotoSansSymbols-VariableFont_wght.ttf")]
    symbols: Handle<Font>,
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontType {
    #[default]
    Default,
    Symbols,
}

#[derive(Message)]
pub struct SpawnLoadingText {
    pub text: String,
}

#[derive(Message)]
pub struct SpawnText {
    pub text: String,
    pub font: FontType,
    pub state: Option<PauseState>,
}

fn on_spawn_loading_text(mut messages: MessageReader<SpawnLoadingText>, mut commands: Commands) {
    for message in messages.read() {
        spawn_fullscreen_text(
            &message.text,
            Some(AppLoadingState::Loading),
            TextFont::from_font_size(DEFAULT_TEXT_SIZE),
            &mut commands,
        );
    }
}

fn on_spawn_text(
    mut messages: MessageReader<SpawnText>,
    fonts: Res<FontsAsssets>,
    mut commands: Commands,
) {
    for message in messages.read() {
        const SIZE: f32 = 48.0;
        let font: TextFont = match message.font {
            FontType::Default => TextFont::from_font_size(SIZE),
            FontType::Symbols => TextFont {
                font: fonts.symbols.clone().into(),
                font_size: FontSize::Px(SIZE),
                ..default()
            },
        };
        spawn_fullscreen_text(&message.text, message.state, font, &mut commands);
    }
}

fn spawn_fullscreen_text<S: States>(
    text: &str,
    state: Option<S>,
    font: TextFont,
    commands: &mut Commands,
) {
    let mut entity = commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Pickable::IGNORE,
        children![(
            Text::new(text),
            font,
            TextLayout::new(Justify::Center, LineBreak::NoWrap),
            Pickable::IGNORE
        )],
    ));

    if let Some(state) = state {
        entity.insert(DespawnOnExit(state));
    }
}
