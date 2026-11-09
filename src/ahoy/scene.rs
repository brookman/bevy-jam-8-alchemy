use crate::loading::{AppLoadingState, LoadCollection};
use bevy::light::atmosphere::ScatteringMedium;
use bevy::light::light_consts::lux;
use bevy::light::{Atmosphere, CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
#[cfg(feature = "dev")]
use bevy_inspector_egui::prelude::*;
use bevy_landmass::{
    Archipelago3d, ArchipelagoOptions, ArchipelagoRef3d, FromAgentRadius, Island,
    PointSampleDistance3d,
};
use bevy_rerecast::Navmesh;
use landmass_rerecast::{Island3dBundle, NavMeshHandle3d};
use std::f32::consts::TAU;

pub fn plugin(app: &mut App) {
    app.load_collection::<SceneAssets>()
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .insert_resource(GlobalAmbientLight::NONE)
        .add_systems(
            OnEnter(AppLoadingState::Loaded),
            (spawn_scene, spawn_light, spawn_atmosphere, spawn_crosshair),
        )
        .add_systems(Update, turn_sun);
}

#[derive(AssetCollection, Resource)]
pub struct SceneAssets {
    #[asset(path = "models/alchemy.blend#Scene0")]
    alchemy_scene: Handle<WorldAsset>,
    #[asset(path = "models/alchemy.nav")]
    alchemy_navmesh: Handle<Navmesh>,
    #[asset(path = "sprites/crosshair.png")]
    crosshair_texture: Handle<Image>,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[type_path = "game"]
pub struct MainScene;

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
        Name::new("Scene"),
        MainScene,
        WorldAssetRoot(assets.alchemy_scene.clone()),
    ));

    let archipelago = commands
        .spawn((
            Name::new("Archipelago"),
            Archipelago3d::new(ArchipelagoOptions {
                point_sample_distance: PointSampleDistance3d {
                    distance_above: 0.5,
                    distance_below: 0.5,
                    ..PointSampleDistance3d::from_agent_radius(0.15)
                },
                ..ArchipelagoOptions::from_agent_radius(0.5)
            }),
        ))
        .id();

    commands.spawn((
        Name::new("Island"),
        Island3dBundle {
            island: Island,
            archipelago_ref: ArchipelagoRef3d::new(archipelago),
            nav_mesh: NavMeshHandle3d(assets.alchemy_navmesh.clone()),
        },
    ));
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        Transform::from_xyz(1.0, 1.0, 0.0).looking_at(vec3(0.0, 0.0, 0.0), Vec3::Y),
        DirectionalLight {
            shadow_maps_enabled: true,
            illuminance: lux::CIVIL_TWILIGHT,
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
