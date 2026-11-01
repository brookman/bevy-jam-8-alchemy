use crate::ahoy::input::PlayerInput;
use crate::ahoy::stable_ground::StableGround;
use crate::input::Sandbox;
use crate::loading::AppState;
use avian3d::prelude::*;
use bevy::anti_alias::taa::TemporalAntiAliasing;
use bevy::camera::{Exposure, Hdr};
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::pbr::ScreenSpaceAmbientOcclusion;
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy_ahoy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::LoadingStateAppExt;
use bevy_asset_loader::prelude::{ConfigureLoadingState, LoadingStateConfig};
use bevy_enhanced_input::prelude::*;

pub fn plugin(app: &mut App) {
    app.configure_loading_state(
        LoadingStateConfig::new(AppState::Loading).load_collection::<PlayerAssets>(),
    )
    .add_systems(OnEnter(AppState::Running), spawn_player);
}

#[derive(AssetCollection, Resource)]
struct PlayerAssets {
    #[asset(path = "environment_maps/voortrekker_interior_1k_diffuse.ktx2")]
    diffuse_map: Handle<Image>,
    #[asset(path = "environment_maps/voortrekker_interior_1k_specular.ktx2")]
    specular_map: Handle<Image>,
}

fn spawn_player(mut commands: Commands, assets: Res<PlayerAssets>) {
    let player = commands
        .spawn_scene(bsn! {
            CharacterController::default()
            // The KCC currently behaves best when using a cylinder
            Collider::cylinder(0.7, 1.8)
            Transform::from_xyz(0.0, 20.0, 0.0)
            StableGround::default()
            // Configure inputs. The actions `Movement`, `Jump`, etc. are provided by Ahoy, you just need to bind them.
            PlayerInput
            Sandbox
            Actions<Sandbox> [
                Action::<Jump>::new()
                Bindings [
                    Binding(KeyCode::Space)
                    --
                    Binding(GamepadButton::South)
                ]
                --
                Action::<Crouch>::new()
                Bindings [
                    Binding(KeyCode::ControlLeft)
                    --
                    Binding(GamepadButton::LeftTrigger2)
                ]
                --
                Action::<Movement>::new()
                Bindings [
                    {preset::cardinal::wasd_keys()}
                    --
                    {preset::axial::left_stick()}
                ]
                --
                Action::<RotateCamera>::new()
                Bindings [
                    Binding::mouse_motion()
                    ~Scale::splat(0.2)
                    --
                    {preset::axial::new(
                        (GamepadAxis::RightStickX, Scale::splat(4.0), DeadZone::default()),
                        (GamepadAxis::RightStickY, Scale::splat(4.0),DeadZone::default()),
                    )}
                ]
            ]
        })
        .id();

    // Camera
    commands.spawn((
        CharacterControllerCameraOf::new(player),
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
