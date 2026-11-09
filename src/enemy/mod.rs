use crate::ahoy::player::PlayerComponent;
use crate::loading::{AppLoadingState, LoadCollection};
use crate::physics::collider::InitialTransform;
use avian_rerecast::AvianBackendPlugin;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_landmass::{AgentDesiredVelocity3d, AgentTarget3d, Landmass3dPlugin};
use bevy_rerecast::NavmeshPlugins;
use landmass_rerecast::LandmassRerecastPlugin;
use std::time::Duration;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        NavmeshPlugins::default(),
        AvianBackendPlugin::default(),
        Landmass3dPlugin::default(),
        LandmassRerecastPlugin::default(),
    ))
    .load_collection::<EnemyAssets>()
    .add_systems(OnEnter(AppLoadingState::Loaded), init)
    .add_systems(
        Update,
        (init_nav_mesh, spawn_enemy, update_enemy_target, move_enemy)
            .chain()
            .run_if(in_state(AppLoadingState::Loaded)),
    );
}

#[derive(AssetCollection, Resource)]
struct EnemyAssets {
    #[asset(path = "models/alchemy.blend#Scene1")]
    enemy: Handle<WorldAsset>,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[type_path = "game"]
pub struct Enemy;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[type_path = "game"]
pub struct EnemyRoot;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[type_path = "game"]
pub struct EnemyAgent;

#[derive(Component, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "game"]
pub struct EnemySpawner {
    spawn_timer: Timer,
}

impl Default for EnemySpawner {
    fn default() -> Self {
        Self {
            spawn_timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
        }
    }
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[type_path = "game"]
pub struct NavMeshMarker;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[type_path = "game"]
pub struct NavObstacle;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[type_path = "game"]
pub struct NavTarget;

// #[derive(Component)]
// pub struct NavPath {
//     pub initialized: bool,
//     pub current: Vec3,
//     pub next: Vec<Vec3>,
// }

fn init(mut commands: Commands) {
    commands.spawn(());
}

fn init_nav_mesh(
    nav_mesh_markers: Query<Entity, With<NavMeshMarker>>,
    children: Query<&Children>,
    mesh_3ds: Query<&Mesh3d>,
    meshes: Res<Assets<Mesh>>,
    mut commands: Commands,
) {
    for entity in nav_mesh_markers {
        for child in children.iter_descendants(entity) {
            info!("init_nav_mesh on {:?}", child);

            let Ok(mesh_3d) = mesh_3ds.get(child) else {
                continue;
            };

            let Some(mesh) = meshes.get(&mesh_3d.0) else {
                continue;
            };

            // let Some(nav_mesh) = NavMesh::from_bevy_mesh(mesh) else {
            //     continue;
            // };

            // commands.entity(child).insert((
            //     ManagedNavMesh::from_id(0),
            //     NavMeshSettings {
            //         fixed: Triangulation::from_mesh(nav_mesh.get().as_ref(), 0),
            //         build_timeout: Some(5.0),
            //         upward_shift: 0.5,
            //         merge_steps: 2,
            //         agent_radius: 0.2,
            //         //agent_radius_on_outer_edge: true,
            //         ..default()
            //     },
            //     NavMeshUpdateMode::Direct,
            //     Transform::from_rotation(Quat::from_axis_angle(Vec3::X, TAU * 0.25)),
            // ));
        }
        commands.entity(entity).remove::<NavMeshMarker>();
    }
}

fn spawn_enemy(
    spawners: Query<(&mut EnemySpawner, &Transform)>,
    assets: Res<EnemyAssets>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut spawner, transform) in spawners {
        spawner.spawn_timer.tick(time.delta());
        if spawner.spawn_timer.just_finished() {
            commands.spawn((
                WorldAssetRoot(assets.enemy.clone()),
                InitialTransform(transform.clone()),
                EnemyRoot
            ));
        }
    }
}

// fn update_enemy_path(
//     mut enemy: Query<(&Transform, &mut NavPath)>,
//     target: Query<&Transform, With<NavTarget>>,
//     navmeshes: Res<Assets<NavMesh>>,
//     navmesh: Query<&ManagedNavMesh>,
//     //mut gizmos: Gizmos,
// ) {
//     for (transform, mut path) in &mut enemy {
//         let navmesh = navmeshes.get(navmesh.single().unwrap()).unwrap();
//         let target = target.single().unwrap().translation;
//
//         let Some(new_path) = navmesh.transformed_path(transform.translation, target) else {
//             break;
//         };
//
//         if let Some((first, remaining)) = new_path.path.split_first() {
//             let mut remaining = remaining.to_vec();
//             remaining.reverse();
//             *path = NavPath {
//                 initialized: true,
//                 current: *first,
//                 next: remaining,
//             };
//         }
//     }
// }

// fn move_enemy(
//     mut commands: Commands,
//     mut object_query: Query<(&mut Transform, &mut NavPath, Entity)>,
//     time: Res<Time>,
// ) {
//     for (mut transform, mut path, entity) in object_query.iter_mut() {
//         if !path.initialized {
//             continue;
//         }
//         if transform.translation.distance(path.current) >= 0.1 {
//             let move_direction = path.current - transform.translation;
//             transform.translation += move_direction.normalize() * time.delta_secs() * 3.0;
//         } else {
//             if let Some(next) = path.next.pop() {
//                 path.current = next;
//             } else {
//                 commands.entity(entity).remove::<NavPath>();
//                 // let target_entity = object.0.take().unwrap();
//                 // commands.entity(target_entity).despawn();
//             }
//         }
//     }
// }

fn update_enemy_target(
    targets: Query<&mut AgentTarget3d>,
    player: Query<&Transform, With<PlayerComponent>>,
    target_q: Query<&Transform, With<NavTarget>>,
    mut commands: Commands,
) {
    let Ok(player) = player.single() else {
        return;
    };

    for mut target in targets {
        *target = AgentTarget3d::Point(player.translation - Vec3::new(0.0, 0.915, 0.0));
        // match *target {
        //     AgentTarget3d::None => {
        //         info!("update target on enemy_entity {:?}", target);
        //         *target = AgentTarget3d::Point(target_q.single().unwrap().translation);
        //     }
        //     _ => {}
        // }
    }
}

fn move_enemy(
    mut commands: Commands,
    mut object_query: Query<(&mut Transform, &AgentDesiredVelocity3d, Entity)>,
    time: Res<Time>,
) {
    for (mut transform, desired_velocity, entity) in object_query.iter_mut() {
        let vel = desired_velocity.velocity();

        transform.translation += vel * time.delta_secs();

        let pos = transform.translation - vel;
        transform.look_at(pos, Vec3::Y);

        // if transform.translation.distance(target.) >= 0.1 {
        //     let move_direction = path.current - transform.translation;
        //     transform.translation += move_direction.normalize() * time.delta_secs() * 3.0;
        // } else {
        //     if let Some(next) = path.next.pop() {
        //         path.current = next;
        //     } else {
        //         commands.entity(entity).remove::<NavPath>();
        //         // let target_entity = object.0.take().unwrap();
        //         // commands.entity(target_entity).despawn();
        //     }
        // }
    }
}

// fn view_navmesh(
//     mut commands: Commands,
//     navmeshes: Query<Entity, With<ManagedNavMesh>>,
//     mut current: Local<usize>,
// ) {
//     if navmeshes.iter().len() == 0 {
//         info!("no nav meshes");
//         return;
//     }
//     for (i, entity) in navmeshes.iter().sort::<Entity>().enumerate() {
//         commands.entity(entity).remove::<NavMeshDebug>();
//         if i == *current {
//             commands
//                 .entity(entity)
//                 .insert(NavMeshDebug(CYAN_500.into()));
//         }
//     }
//     *current = (*current + 1) % navmeshes.iter().len();
// }
