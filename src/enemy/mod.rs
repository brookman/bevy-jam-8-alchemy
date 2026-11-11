use crate::ahoy::player::PlayerComponent;
use crate::loading::{AppLoadingState, LoadCollection};
use crate::physics::collider::InitialTransform;
use avian_rerecast::AvianBackendPlugin;
use avian3d::prelude::{Forces, PhysicsSystems, ReadRigidBodyForces, WriteRigidBodyForces};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
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
        (
            spawn_enemy,
            update_enemy_target.run_if(on_timer(Duration::from_secs_f32(0.3))),
            move_enemy,
        )
            .chain()
            .run_if(in_state(AppLoadingState::Loaded)),
    )
    .add_systems(
        FixedUpdate,
        move_enemy
            .before(PhysicsSystems::First)
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
pub struct EnemyAgent {
    pub response_time: f32,
    pub max_acceleration: f32,
}

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

fn move_enemy(mut enemies: Query<(Forces, &AgentDesiredVelocity3d, &EnemyAgent, &mut Transform)>) {
    const GRAVITY: f32 = 9.81;

    for (mut forces, desired_velocity, agent, mut transform) in &mut enemies {
        let target = desired_velocity.velocity();
        let current = forces.linear_velocity();
        let velocity_acceleration = (target - current) / agent.response_time;

        let gravity = Vec3::new(0.0, -GRAVITY, 0.0);
        let gravity_compensation = if target.y > 0.0 {
            let direction = target.normalize();
            -gravity.dot(direction) * direction
        } else {
            Vec3::ZERO
        };

        let acceleration =
            (velocity_acceleration + gravity_compensation).clamp_length_max(agent.max_acceleration);

        forces.apply_linear_acceleration(acceleration);

        let horizontal = Vec3::new(target.x, 0.0, target.z);
        if horizontal.length_squared() > 0.001 {
            transform.look_to(horizontal, Vec3::Y);
        }
    }
}
