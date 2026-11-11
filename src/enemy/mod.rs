use crate::ahoy::player::PlayerComponent;
use crate::loading::{AppLoadingState, LoadCollection};
use crate::physics::collider::InitialTransform;
use avian_rerecast::AvianBackendPlugin;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_landmass::{
    AgentDesiredVelocity3d, AgentTarget3d, Archipelago3d, Landmass3dPlugin, PointSampleDistance3d,
};
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
        (spawn_enemy, update_enemy_target, move_enemy)
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

fn init(mut commands: Commands) {
    commands.spawn(());
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
                EnemyRoot,
            ));
        }
    }
}

fn update_enemy_target(
    targets: Query<&mut AgentTarget3d>,
    player: Query<&Transform, With<PlayerComponent>>,
    archipelago: Query<&Archipelago3d>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };

    let Ok(archipelago) = archipelago.single() else {
        return;
    };

    let distance = PointSampleDistance3d {
        horizontal_distance: 1.0,
        distance_above: 0.1,
        distance_below: 5.0,
        vertical_preference_ratio: 1.0,
        animation_link_max_vertical_distance: 0.5,
    };

    let nearest = archipelago
        .sample_point(player_transform.translation, &distance)
        .ok()
        .map(|sample| sample.point());

    let Some(nearest) = nearest else {
        return;
    };

    for mut target in targets {
        *target = AgentTarget3d::Point(nearest);
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
