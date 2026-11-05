use crate::loading::{AppState, LoadCollection};
use bevy::light::atmosphere::ScatteringMedium;
use bevy::light::light_consts::lux;
use bevy::light::{Atmosphere, CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
#[cfg(feature = "dev")]
use bevy_inspector_egui::prelude::*;
use std::f32::consts::TAU;

pub fn plugin(app: &mut App) {
    app.load_collection::<SceneAssets>()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .insert_resource(GlobalAmbientLight::NONE)
        .add_systems(
            OnEnter(AppState::Running),
            (spawn_scene, spawn_light, spawn_atmosphere, spawn_crosshair),
        )
        .add_systems(Update, turn_sun);
}

#[derive(AssetCollection, Resource)]
struct SceneAssets {
    #[asset(path = "maps/new_playground.blend#Scene0")]
    playground: Handle<WorldAsset>,
    #[asset(path = "models/alchemy.blend#Scene0")]
    table: Handle<WorldAsset>,
    #[asset(path = "sprites/crosshair.png")]
    crosshair_texture: Handle<Image>,
}

fn spawn_scene(mut commands: Commands, assets: Res<SceneAssets>) {
    // commands.spawn((
    //     Name::new("Scene"),
    //     WorldAssetRoot(assets.playground.clone()),
    //     RigidBody::Static,
    //     ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh)
    //         .with_default_layers(CollisionLayers::new(
    //             CollisionLayer::Obstacle,
    //             [CollisionLayer::Player, CollisionLayer::Prop],
    //         )),
    // ));
    commands.spawn((
        Name::new("Table"),
        Transform::from_xyz(0.0, 7.75, -3.0),
        WorldAssetRoot(assets.table.clone()),
    ));
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        Transform::from_xyz(0.0, 1.0, 0.0).looking_at(vec3(1.0, -2.0, -2.0), Vec3::Y),
        DirectionalLight {
            shadow_maps_enabled: true,
            illuminance: lux::OVERCAST_DAY,
            ..default()
        },
        CascadeShadowConfigBuilder {
            maximum_distance: 500.0,
            overlap_proportion: 0.4,
            ..default()
        }
        .build(),
    ));
}

fn spawn_atmosphere(
    mut commands: Commands,
    mut scattering_mediums: ResMut<Assets<ScatteringMedium>>,
) {
    commands.spawn(Atmosphere::earth(
        scattering_mediums.add(ScatteringMedium::default()),
    ));
}

fn spawn_crosshair(mut commands: Commands, assets: Res<SceneAssets>) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                ImageNode::new(assets.crosshair_texture.clone())
                    .with_color(Color::WHITE.with_alpha(0.3)),
            );
        });
}

fn turn_sun(mut suns: Query<&mut Transform, With<DirectionalLight>>, time: Res<Time>) {
    for mut transform in suns.iter_mut() {
        transform.rotation =
            Quat::from_rotation_x(
                -((-time.elapsed_secs() / 100.0) + TAU / 8.0).sin().abs() * TAU / 2.05,
            ) * Quat::from_rotation_y(((-time.elapsed_secs() / 100.0) + 1.0).sin());
    }
}
