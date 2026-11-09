use crate::loading::LoadCollection;
use crate::physics::collider::CollisionLayer;
use avian3d::prelude::SpatialQueryFilter;
use bevy::anti_alias::taa::TemporalAntiAliasing;
use bevy::app::App;
use bevy::asset::Handle;
use bevy::camera::{Camera3d, Exposure, Hdr, PerspectiveProjection, Projection};
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::image::Image;
use bevy::light::EnvironmentMapLight;
use bevy::pbr::ScreenSpaceAmbientOcclusion;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy_ahoy::camera::CharacterControllerCameraOf;
use bevy_ahoy::{
    CharacterController, PickupConfig, PickupHoldConfig, PickupPullConfig, PickupThrowConfig,
};
use bevy_asset_loader::asset_collection::AssetCollection;
use crate::physics::collider::CollisionLayer::Player;

pub fn plugin(app: &mut App) {
    app.load_collection::<CameraAssets>()
        .add_observer(spawn_camera);
}

#[derive(AssetCollection, Resource)]
struct CameraAssets {
    #[asset(path = "environment_maps/voortrekker_interior_1k_diffuse.ktx2")]
    diffuse_map: Handle<Image>,
    #[asset(path = "environment_maps/voortrekker_interior_1k_specular.ktx2")]
    specular_map: Handle<Image>,
}

fn spawn_camera(
    controller: On<Add<CharacterController>>,
    mut commands: Commands,
    assets: Res<CameraAssets>,
) {
    commands.spawn((
        CharacterControllerCameraOf::new(controller.entity),
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            far: 200.0,
            fov: 70.0_f32.to_radians(),
            ..default()
        }),
        Hdr,
        Msaa::Off,
        ScreenSpaceAmbientOcclusion::default(),
        TemporalAntiAliasing::default(),
        EnvironmentMapLight {
            diffuse_map: assets.diffuse_map.clone(),
            specular_map: assets.specular_map.clone(),
            intensity: 500.0,
            ..default()
        },
        Exposure { ev100: 9.0 },
        Tonemapping::TonyMcMapface,
        Bloom::default(),
        DebandDither::Enabled,
        PickupConfig {
            prop_filter: SpatialQueryFilter::from_mask(CollisionLayer::Prop),
            actor_filter: SpatialQueryFilter::from_mask(CollisionLayer::Player),
            obstacle_filter: SpatialQueryFilter::from_mask(CollisionLayer::Obstacle),
            hold: PickupHoldConfig {
                distance_to_allow_holding: 2.0,
                //preferred_distance: 1.0,
                linear_velocity_easing: 0.0,
                angular_velocity_easing: 0.0,
                //temporary_prop_mass: 0.1,
                min_distance: 0.6,
                max_linear_speed: 100.0,
                max_angular_speed: 100.0,
                ..default()
            },
            pull: PickupPullConfig {
                impulse: 0.0,
                ..default()
            },
            throw: PickupThrowConfig {
                cutoff_mass_for_slowdown: 50.0,
                linear_speed_range: 0.0..=10.0,
                angular_speed_range: 0.0..=3.0,
            },
            ..default()
        },
    ));
}
