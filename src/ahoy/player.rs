use crate::ahoy::input::PlayerInput;
use crate::ahoy::stable_ground::StableGround;
use crate::input::InGame;
use crate::loading::AppState;
use crate::physics::CollisionLayer;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_ahoy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Running), spawn_player);
}

fn spawn_player(mut commands: Commands) {
    let controller = CharacterController {
        filter: SpatialQueryFilter::from_mask(CollisionLayer::Obstacle),
        speed: 6.0,
        ..default()
    };
    commands.spawn_scene(bsn! {
        # Player
        Transform::from_xyz(0.0, 20.0, 0.0)
        Collider::cylinder(0.35, 1.8)
        //CollisionLayers::new(CollisionLayer::Player, CollisionLayer::Obstacle)
        controller
        RigidBody::Kinematic
        Mass(90.0)
        StableGround::default()

        PlayerInput
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
