use crate::loading::{AppLoadingState, LoadCollection};
use crate::physics::collider::InitialTransform;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use std::time::Duration;

pub fn plugin(app: &mut App) {
    app.load_collection::<EnemyAssets>()
        .add_systems(OnEnter(AppLoadingState::Loaded), init)
        .add_systems(
            Update,
            (spawn_enemy, update_enemy).run_if(in_state(AppLoadingState::Loaded)),
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

#[derive(Component, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "game"]
pub struct EnemySpawner {
    spawn_timer: Timer,
}

impl Default for EnemySpawner {
    fn default() -> Self {
        Self {
            spawn_timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
        }
    }
}

fn init() {}

fn spawn_enemy(
    spawners: Query<(&mut EnemySpawner, &Transform)>,
    assets: Res<EnemyAssets>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut spawner, transform) in spawners {
        spawner.spawn_timer.tick(time.delta());
        if spawner.spawn_timer.just_finished() {
            info!("spawner transform: {:?}", transform);
            commands.spawn((
                WorldAssetRoot(assets.enemy.clone()),
                InitialTransform(transform.clone()),
            ));
        }
    }
}

fn update_enemy() {}
