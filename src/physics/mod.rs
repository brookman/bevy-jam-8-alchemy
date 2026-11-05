use crate::input::ClearBalls;
use crate::loading::{AppLoadingState, LoadCollection};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_enhanced_input::prelude::Start;

pub(crate) mod collider;
mod damping;
mod layer;
mod potion;

const DESPAWN_HEIGHT: f32 = -30.0;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default(),
        collider::plugin,
        damping::plugin,
        potion::plugin,
        #[cfg(feature = "dev")]
        PhysicsDebugPlugin,
    ))
    .load_collection::<BallAssets>()
    .add_systems(OnEnter(AppLoadingState::Loaded), spawn_ball_prop)
    .add_systems(
        Update,
        (despawn_fallen).run_if(in_state(AppLoadingState::Loaded)),
    )
    .add_observer(spawn_ball_components)
    .add_observer(clear_balls);
}

#[derive(AssetCollection, Resource)]
struct BallAssets {
    #[asset(path = "models/demo.blend#Scene1")]
    scene: Handle<WorldAsset>,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[type_path = "game"]
pub struct Ball;

#[derive(Component)]
struct BallRoot;

fn spawn_ball_prop(mut commands: Commands, assets: Res<BallAssets>) {
    commands.spawn((BallRoot, WorldAssetRoot(assets.scene.clone())));
}

fn spawn_ball_components(ball: On<Add<Ball>>, mut commands: Commands) {
    let position = Vec3::new(0.0, 20.0, -5.0);
    commands.entity(ball.entity).insert((
        Transform::from_translation(position),
        //TransformInterpolation,
        // ColliderConstructor::Sphere { radius: 0.15 },
        // CollisionLayers::new(
        //     CollisionLayer::Prop,
        //     [CollisionLayer::Obstacle, CollisionLayer::Prop],
        // ),
    ));
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
    _: On<Start<ClearBalls>>,
    mut commands: Commands,
    roots: Query<Entity, With<BallRoot>>,
) {
    for root in &roots {
        commands.entity(root).despawn();
    }
}

// fn random_signed() -> f32 {
//     fastrand::f32().mul_add(2.0, -1.0)
// }

// fn launch_ball(
//     ready: On<WorldInstanceReady>,
//     mut commands: Commands,
//     roots: Query<&InitialVelocity>,
//     children: Query<&Children>,
//     bodies: Query<(), With<RigidBody>>,
//     meshes: Query<(), With<Mesh3d>>,
// ) {
//     let Ok(InitialVelocity(velocity)) = roots.get(ready.entity) else {
//         return;
//     };
//     for entity in children.iter_descendants(ready.entity) {
//         if bodies.contains(entity) {
//             commands.entity(entity).insert(LinearVelocity(*velocity));
//         }
//         if meshes.contains(entity) {
//             // Don't let balls steal the cube's hover.
//             commands.entity(entity).insert(Pickable::IGNORE);
//         }
//     }
//     commands.entity(ready.entity).remove::<InitialVelocity>();
// }

// fn add_spawn_timer(
//     add: On<Add<BallSpawner>>,
//     mut commands: Commands,
//     spawners: Query<&BallSpawner>,
// ) {
//     let Ok(spawner) = spawners.get(add.entity) else {
//         return;
//     };
//     let interval = spawner.interval.max(0.01);
//     commands
//         .entity(add.entity)
//         .insert(SpawnTimer(Timer::from_seconds(
//             interval,
//             TimerMode::Repeating,
//         )));
// }

// fn spawn_timed(
//     mut commands: Commands,
//     time: Res<Time>,
//     assets: Res<BallAssets>,
//     mut spawners: Query<(&BallSpawner, &mut SpawnTimer, &GlobalTransform)>,
// ) {
//     for (spawner, mut timer, transform) in &mut spawners {
//         for _ in 0..timer.0.tick(time.delta()).times_finished_this_tick() {
//             spawn_ball(&mut commands, &assets, spawner, transform);
//         }
//     }
// }
