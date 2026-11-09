use crate::ahoy::stable_ground::StableGround;
use crate::input::InGame;
use crate::physics::collider::CollisionLayer;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_ahoy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_observer(spawn_player);
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[type_path = "game"]
pub struct PlayerSpawner {}

#[derive(Component, Reflect, Clone, Debug, Default)]
#[reflect(Component, Default)]
#[type_path = "game"]
pub struct PlayerComponent;

fn spawn_player(
    _: On<Add<PlayerSpawner>>,
    player_spawner: Query<&Transform, With<PlayerSpawner>>,
    mut commands: Commands,
) {
    let controller = CharacterController {
        filter: SpatialQueryFilter::from_mask(CollisionLayer::Obstacle),
        speed: 6.0,
        jump_height: 0.75,
        ..default()
    };

    const PLAYER_HEIGHT: f32 = 1.8;

    let transform = if let Ok(t) = player_spawner.single() {
        t.with_translation(t.translation + Vec3::new(0.0, PLAYER_HEIGHT, 0.0))
    } else {
        Transform::default()
    };

    commands.spawn_scene(bsn! {
        Name::new("Player")
        PlayerComponent
        transform
        Collider::cylinder(0.35, PLAYER_HEIGHT)
        //CollisionLayers::new(CollisionLayer::Player, CollisionLayer::Obstacle)
        controller
        RigidBody::Kinematic
        Mass(90.0)
        StableGround::default()

        InGame
        Actions<InGame> [
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
            --
            Action::<PullObject>::new()
            ActionSettings {
                consume_input: true,
                require_reset: false,
                accumulation: Accumulation::default()
            }
            Press::default()
            Bindings [
                    Binding(MouseButton::Right)
            ]
            --
            Action::<DropObject>::new()
            ActionSettings {
                consume_input: true,
                require_reset: false,
                accumulation: Accumulation::default()
            }
            Press::default()
            Bindings [
                    Binding(MouseButton::Right)
            ]
             --
            Action::<ThrowObject>::new()
            ActionSettings {
                consume_input: true,
                require_reset: false,
                accumulation: Accumulation::default()
            }
            Press::default()
            Bindings [
                    Binding(MouseButton::Left)
            ]
        ]
    });
}
