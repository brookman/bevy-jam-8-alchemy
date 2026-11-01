use crate::ahoy::input::PlayerInput;
use crate::ahoy::stable_ground::StableGround;
use crate::input::Sandbox;
use crate::loading::AppState;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_ahoy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Running), spawn_player);
}

fn spawn_player(mut commands: Commands) {
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
}
