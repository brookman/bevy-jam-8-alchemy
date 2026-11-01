use crate::loading::LoadCollection;
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
use bevy_ahoy::CharacterController;
use bevy_ahoy::camera::CharacterControllerCameraOf;
use bevy_asset_loader::asset_collection::AssetCollection;

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
    controller_added: On<Add<CharacterController>>,
    mut commands: Commands,
    assets: Res<CameraAssets>,
) {
    commands.spawn((
        CharacterControllerCameraOf::new(controller_added.entity),
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
            intensity: 600.0,
            ..default()
        },
        Exposure { ev100: 9.0 },
        Tonemapping::TonyMcMapface,
        Bloom::default(),
        DebandDither::Enabled,
    ));
}
