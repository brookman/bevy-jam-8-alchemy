use crate::ahoy::stable_ground::StableGround;
use avian3d::prelude::*;
use bevy::ecs::world::FilteredEntityRef;
use bevy::{platform::collections::HashSet, prelude::*};
use bevy_ahoy::{CharacterControllerOutput, CharacterControllerState, prelude::*};
use bevy_enhanced_input::prelude::{Release, *};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_ui)
        .add_systems(Update, update_debug_text)
        .add_observer(reset_player)
        .add_observer(toggle_debug)
        .add_input_context::<DebugInput>();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct DebugText;

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Node::default(),
        Text::default(),
        Visibility::Hidden,
        DebugText,
    ));
    commands.spawn((
        Node {
            justify_self: JustifySelf::End,
            justify_content: JustifyContent::End,
            align_self: AlignSelf::End,
            padding: UiRect::all(px(10.0)),
            ..default()
        },
        Text::new(
            "Controls:\nWASD: move\nSpace: jump\nCtrl: crouch\nEsc: free mouse\nR: reset position\nBacktick: Toggle Debug Menu",
        ),
    ));
    commands.spawn((
        DebugInput,
        actions!(DebugInput[
            (
                Action::<Reset>::new(),
                bindings![KeyCode::KeyR, GamepadButton::Select],
                Release::default(),
            ),
            (
                Action::<ToggleDebug>::new(),
                bindings![KeyCode::Backquote, GamepadButton::Start],
                Release::default(),
            ),
        ]),
    ));
}

fn update_debug_text(
    mut text: Single<&mut Text, With<DebugText>>,
    kcc: Single<
        (
            &CharacterControllerState,
            &CharacterControllerOutput,
            &LinearVelocity,
            &CollidingEntities,
            &ColliderAabb,
            &StableGround,
        ),
        (With<CharacterController>, With<CharacterControllerCamera>),
    >,
    camera: Single<&Transform, With<Camera>>,
    names: Query<NameOrEntity>,
) {
    let (state, output, velocity, colliding_entities, aabb, stable_ground) = kcc.into_inner();
    let velocity = **velocity;
    let speed = velocity.length();
    let horizontal_speed = velocity.xz().length();
    let camera_position = camera.translation;
    let collisions = names
        .iter_many(
            output
                .touching_entities
                .iter()
                .map(|e| e.entity)
                .collect::<HashSet<_>>(),
        )
        .map(|name| {
            let name = name.unwrap();
            name.name
                .map(|n| format!("{} ({})", name.entity, n))
                .unwrap_or_else(|| format!("{}", name.entity))
        })
        .collect::<Vec<_>>();
    let real_collisions = names
        .iter_many(colliding_entities.iter())
        .map(|name| {
            let name = name.unwrap();
            name.name
                .map(|n| format!("{} ({})", name.entity, n))
                .unwrap_or_else(|| format!("{}", name.entity))
        })
        .collect::<Vec<_>>();
    let ground = state
        .grounded
        .and_then(|ground| names.get(ground.entity).ok())
        .map(|name| {
            name.name
                .map(|n| format!("{} ({})", name.entity, n))
                .unwrap_or(format!("{}", name.entity))
        });
    let stable_ground = stable_ground.previous.back();
    text.0 = format!(
        "Speed: {speed:.3}\nHorizontal Speed: {horizontal_speed:.3}\nVelocity: [{:.3}, {:.3}, {:.3}]\nCamera Position: [{:.3}, {:.3}, {:.3}]\nCollider Aabb:\n  min:[{:.3}, {:.3}, {:.3}]\n  max:[{:.3}, {:.3}, {:.3}]\nReal Collisions: {:#?}\nCollisions: {:#?}\nGround: {:?}\nLast Stable Ground: {:?}",
        velocity.x,
        velocity.y,
        velocity.z,
        camera_position.x,
        camera_position.y,
        camera_position.z,
        aabb.min.x,
        aabb.min.y,
        aabb.min.z,
        aabb.max.x,
        aabb.max.y,
        aabb.max.z,
        real_collisions,
        collisions,
        ground,
        stable_ground
    );
}

#[derive(Component, Default)]
struct DebugInput;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub(super) struct Reset;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub(super) struct ToggleDebug;

fn reset_player(_fire: On<Fire<Reset>>, mut commands: Commands) {
    commands.run_system_cached(reset_player_inner);
}

fn toggle_debug(
    _fire: On<Fire<ToggleDebug>>,
    mut visibility: Single<&mut Visibility, With<DebugText>>,
) {
    **visibility = match **visibility {
        Visibility::Hidden => Visibility::Inherited,
        _ => Visibility::Hidden,
    };
}

fn reset_player_inner(
    world: &mut World,
    // Mutating the player `Transform` breaks on web for some reason? I blame interpolation.
    mut player: Local<QueryState<(&mut Position, &mut LinearVelocity), With<CharacterController>>>,
    mut camera: Local<QueryState<&mut Transform, (With<Camera3d>, Without<CharacterController>)>>,
    mut spawner: Local<QueryState<&Transform, (Without<CharacterController>, Without<Camera3d>)>>,
) {
    let component_id = {
        let type_registry = world.resource::<AppTypeRegistry>().read();
        let Some(registration) = type_registry.get_with_short_type_path("SpawnPlayer") else {
            return;
        };
        let type_id = registration.type_id();
        let Some(component_id) = world.components().get_id(type_id) else {
            return;
        };
        component_id
    };
    let mut query = QueryBuilder::<FilteredEntityRef>::new(world)
        .ref_id(component_id)
        .build();
    let Some(spawn_entity) = query.iter(world).map(|e| e.entity()).next() else {
        return;
    };
    let Ok(spawner_transform) = spawner.get(world, spawn_entity).copied() else {
        return;
    };

    let Ok((mut position, mut velocity)) = player.single_mut(world) else {
        return;
    };
    **velocity = Vec3::ZERO;
    position.0 = spawner_transform.translation;
    let Ok(mut camera_transform) = camera.single_mut(world) else {
        return;
    };
    camera_transform.rotation = Quat::IDENTITY;
}
