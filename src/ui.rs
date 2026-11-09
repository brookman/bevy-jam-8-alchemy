use crate::{
    loading::{AppLoadingState, LoadCollection},
    pause::PauseState,
};
use bevy::{prelude::*, ui_widgets};
use bevy_asset_loader::asset_collection::AssetCollection;

const DEFAULT_TEXT_SIZE: f32 = 48.0;

pub fn plugin(app: &mut App) {
    app.load_collection::<FontsAsssets>()
        .add_message::<SpawnLoadingText>()
        .add_message::<SpawnText>()
        .add_message::<SpawnMenu>()
        .add_systems(
            Update,
            (
                on_spawn_loading_text,
                on_spawn_text.run_if(resource_exists::<FontsAsssets>),
                on_spawn_menu.run_if(resource_exists::<FontsAsssets>),
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

impl FontType {
    fn default_text_font() -> TextFont {
        TextFont::from_font_size(DEFAULT_TEXT_SIZE)
    }
    fn default_text_font2() -> TextFont {
        TextFont::from_font_size(16.0)
    }

    fn to_text_font(&self, font_assets: &FontsAsssets) -> TextFont {
        match self {
            FontType::Default => Self::default_text_font(),
            FontType::Symbols => TextFont {
                font: font_assets.symbols.clone().into(),
                font_size: FontSize::Px(DEFAULT_TEXT_SIZE),
                ..default()
            },
        }
    }
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

#[derive(Message)]
pub struct SpawnMenu {
    pub text: String,
    pub text2: String,
    pub font: FontType,
    pub state: Option<PauseState>,
}

fn on_spawn_loading_text(mut messages: MessageReader<SpawnLoadingText>, mut commands: Commands) {
    for message in messages.read() {
        spawn_fullscreen_text(
            &message.text,
            "",
            Some(AppLoadingState::Loading),
            FontType::default_text_font(),
            &mut commands,
        );
    }
}

fn on_spawn_text(
    mut messages: MessageReader<SpawnText>,
    font_assets: Res<FontsAsssets>,
    mut commands: Commands,
) {
    for message in messages.read() {
        spawn_fullscreen_text(
            &message.text,
            "",
            message.state,
            message.font.to_text_font(&font_assets),
            &mut commands,
        );
    }
}

fn on_spawn_menu(
    mut messages: MessageReader<SpawnMenu>,
    font_assets: Res<FontsAsssets>,
    mut commands: Commands,
) {
    for message in messages.read() {
        let root = spawn_fullscreen_text(
            &message.text,
            &message.text2,
            message.state,
            message.font.to_text_font(&font_assets),
            &mut commands,
        );
        commands.entity(root).with_children(|parent| {
            parent
                .spawn((
                    ui_widgets::Button,
                    Node {
                        padding: UiRect::axes(px(24), px(12)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    children![(
                        Text::new("Click to play"),
                        TextFont::default(),
                        TextColor(Color::WHITE),
                    )],
                ))
                .observe(on_press_continue);
        });
    }
}

fn on_press_continue(_: On<ui_widgets::Activate>, mut next: ResMut<NextState<PauseState>>) {
    next.set(PauseState::Running);
}

fn spawn_fullscreen_text<S: States>(
    text: &str,
    text2: &str,
    state: Option<S>,
    font: TextFont,
    commands: &mut Commands,
) -> Entity {
    let mut entity = commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            row_gap: px(20),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,

            ..default()
        },
        Pickable::IGNORE,
        BackgroundColor(Color::srgba(0.2, 0.2, 0.2,0.2)),
        children![
            (
                Text::new(text),
                font,
                TextLayout::new(Justify::Center, LineBreak::NoWrap),
                Pickable::IGNORE
            ),
            (
                Text::new(text2),
                FontType::default_text_font2(),
                TextLayout::new(Justify::Center, LineBreak::NoWrap),
                Pickable::IGNORE
            )
        ],
    ));

    if let Some(state) = state {
        entity.insert(DespawnOnExit(state));
    }
    entity.id()
}
