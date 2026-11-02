//! Balls and their spawners come from Blender (skein components)

use crate::input::{ClearSpheres, DropBurst};
use crate::loading::{AppState, LoadCollection};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::world_serialization::WorldInstanceReady;
use bevy_asset_loader::prelude::*;
use bevy_enhanced_input::prelude::Start;

const DESPAWN_HEIGHT: f32 = -20.0;
const BURST_SIZE: usize = 10;

pub fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default())
        .register_type::<Ball>()
        .register_type::<BallSpawner>()
        .load_collection::<BallAssets>()
        .add_observer(add_spawn_timer)
        .add_observer(launch_ball)
        .add_observer(drop_burst)
        .add_observer(clear_balls)
        .add_systems(OnEnter(AppState::Running), spawn_ball_prop)
        .add_systems(
            Update,
            (spawn_timed, despawn_fallen, update_damping).run_if(in_state(AppState::Running)),
        );
}

#[derive(AssetCollection, Resource)]
struct BallAssets {
    #[asset(path = "models/demo.blend#Scene1")]
    scene: Handle<WorldAsset>,
}

#[derive(Debug, PhysicsLayer, Default)]
pub enum CollisionLayer {
    #[default]
    Default,
    Player,
    Prop,
}

/// Skein marker for the ball's physics body (used for cleanup and impact sounds).
#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[type_path = "game"]
pub struct Ball;

#[derive(Component, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "game"]
pub struct BallSpawner {
    pub interval: f32,
    pub max_speed: f32,
}

impl Default for BallSpawner {
    fn default() -> Self {
        Self {
            interval: 0.5,
            max_speed: 1.0,
        }
    }
}

#[derive(Component)]
struct SpawnTimer(Timer);

#[derive(Component)]
struct BallRoot;

#[derive(Component)]
struct InitialVelocity(Vec3);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[require(AngularDamping, CollidingEntities)]
#[type_path = "game"]
struct AdaptiveAngularDamping {
    default: f32,
    colliding: f32,
}

fn add_spawn_timer(
    add: On<Add<BallSpawner>>,
    mut commands: Commands,
    spawners: Query<&BallSpawner>,
) {
    let Ok(spawner) = spawners.get(add.entity) else {
        return;
    };
    let interval = spawner.interval.max(0.01);
    commands
        .entity(add.entity)
        .insert(SpawnTimer(Timer::from_seconds(
            interval,
            TimerMode::Repeating,
        )));
}

fn spawn_timed(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<BallAssets>,
    mut spawners: Query<(&BallSpawner, &mut SpawnTimer, &GlobalTransform)>,
) {
    for (spawner, mut timer, transform) in &mut spawners {
        for _ in 0..timer.0.tick(time.delta()).times_finished_this_tick() {
            spawn_ball(&mut commands, &assets, spawner, transform);
        }
    }
}

fn drop_burst(
    _: On<Start<DropBurst>>,
    mut commands: Commands,
    assets: Option<Res<BallAssets>>,
    spawners: Query<(&BallSpawner, &GlobalTransform)>,
) {
    let Some(assets) = assets else {
        return;
    };
    for (spawner, transform) in &spawners {
        for _ in 0..BURST_SIZE {
            spawn_ball(&mut commands, &assets, spawner, transform);
        }
    }
}

fn spawn_ball(
    commands: &mut Commands,
    assets: &BallAssets,
    spawner: &BallSpawner,
    transform: &GlobalTransform,
) {
    let position = spawn_point(transform, random_unit_box());
    let velocity = random_unit_box() * spawner.max_speed;
    commands.spawn((
        Name::new("Ball"),
        BallRoot,
        InitialVelocity(velocity),
        WorldAssetRoot(assets.scene.clone()),
        Transform::from_translation(position),
    ));
}

fn spawn_ball_prop(mut commands: Commands, assets: Res<BallAssets>) {
    let position = Vec3::new(0.0, 30.0, 0.0);
    let velocity = random_unit_box() * 0.1;
    commands.spawn((
        Name::new("BallProp"),
        BallRoot,
        Ball,
        InitialVelocity(velocity),
        WorldAssetRoot(assets.scene.clone()),
        Transform::from_translation(position),
        ColliderConstructor::Sphere { radius: 0.15 },
        CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL),
        RigidBody::Dynamic,
        Mass(0.5),
        Restitution {
            coefficient: 0.75,
            combine_rule: CoefficientCombine::Average,
        },
        Friction::default(),
        AdaptiveAngularDamping {
            default: 0.1,
            colliding: 4.0,
        },
        TransformInterpolation,
    ));
}

fn update_damping(
    query: Query<(
        &CollidingEntities,
        &AdaptiveAngularDamping,
        &mut AngularDamping,
    )>,
) {
    for (colliding, variable, mut damping) in query {
        damping.0 = if colliding.0.is_empty() {
            variable.default
        } else {
            variable.colliding
        };
    }
}

fn spawn_point(spawner: &GlobalTransform, unit: Vec3) -> Vec3 {
    spawner.transform_point(unit)
}

fn random_unit_box() -> Vec3 {
    Vec3::new(random_signed(), random_signed(), random_signed())
}

fn random_signed() -> f32 {
    fastrand::f32().mul_add(2.0, -1.0)
}

fn launch_ball(
    ready: On<WorldInstanceReady>,
    mut commands: Commands,
    roots: Query<&InitialVelocity>,
    children: Query<&Children>,
    bodies: Query<(), With<RigidBody>>,
    meshes: Query<(), With<Mesh3d>>,
) {
    let Ok(InitialVelocity(velocity)) = roots.get(ready.entity) else {
        return;
    };
    for entity in children.iter_descendants(ready.entity) {
        if bodies.contains(entity) {
            commands.entity(entity).insert(LinearVelocity(*velocity));
        }
        if meshes.contains(entity) {
            // Don't let balls steal the cube's hover.
            commands.entity(entity).insert(Pickable::IGNORE);
        }
    }
    commands.entity(ready.entity).remove::<InitialVelocity>();
}

fn despawn_fallen(
    mut commands: Commands,
    balls: Query<(Entity, &GlobalTransform), With<Ball>>,
    parents: Query<&ChildOf>,
    roots: Query<(), With<BallRoot>>,
) {
    for (ball, transform) in &balls {
        if transform.translation().y < DESPAWN_HEIGHT {
            let root = parents
                .iter_ancestors(ball)
                .find(|&ancestor| roots.contains(ancestor))
                .unwrap_or(ball);
            commands.entity(root).despawn();
        }
    }
}

fn clear_balls(
    _: On<Start<ClearSpheres>>,
    mut commands: Commands,
    roots: Query<Entity, With<BallRoot>>,
) {
    for root in &roots {
        commands.entity(root).despawn();
    }
}
