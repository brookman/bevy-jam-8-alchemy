use crate::ahoy::scene::SceneAssets;
use crate::enemy::{Enemy, EnemyRoot};
use crate::loading::AppLoadingState;
use bevy::app::App;
use bevy::prelude::*;
use bevy::prelude::{Component, Reflect};
use std::time::Duration;

const DURATION: f32 = 0.2;
const RADIUS: f32 = 1.5;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update_explosion).run_if(in_state(AppLoadingState::Loaded)),
    )
    .add_observer(on_add_explosion);
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[type_path = "game"]
pub struct ExplosionMarker;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[type_path = "game"]
pub struct Explosion {
    pub transform: Transform,
    pub radius: f32,
    pub force: f32,
    explosion_timer: Timer,
}

impl Explosion {
    pub fn new(transform: Transform, force: f32) -> Self {
        Self {
            transform,
            radius: RADIUS,
            force,
            explosion_timer: Timer::new(Duration::from_secs_f32(DURATION), TimerMode::Once),
        }
    }
}

// fn spawn_explosions(
//     mut explosions: MessageReader<ExplosionMessage>,
//     assets: Res<SceneAssets>,
//     mut commands: Commands,
// ) {
//     for explosion in explosions.read() {
//         commands.spawn((
//             Name::new("Explosion"),
//             WorldAssetRoot(assets.explosion_scene.clone()),
//             explosion.transform.clone(),
//         ));
//     }
// }

fn on_add_explosion(
    explosion: On<Add<Explosion>>,
    explosion_query: Query<&Explosion>,
    assets: Res<SceneAssets>,
    mut commands: Commands,
) {
    let e = explosion_query.get(explosion.entity).unwrap();
    commands.entity(explosion.entity).insert((
        Name::new("Explosion"),
        WorldAssetRoot(assets.explosion_scene.clone()),
        e.transform.clone().with_scale(Vec3::splat(e.radius * 2.0)),
    ));
}

fn update_explosion(
    explosions: Query<(Entity, &mut Transform, &mut Explosion), Without<Enemy>>,
    time: Res<Time>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    parents: Query<&ChildOf>,
    roots: Query<(), With<EnemyRoot>>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut explosion) in explosions {
        explosion.explosion_timer.tick(time.delta());
        let f = explosion.explosion_timer.elapsed_secs() / DURATION;
        transform.scale = Vec3::splat(explosion.radius * 2.0 * f);
        if explosion.explosion_timer.is_finished() {
            for (enemy, enemy_transform) in enemies {
                if enemy_transform.translation.distance(transform.translation) <= explosion.radius {
                    let root = parents
                        .iter_ancestors(enemy)
                        .find(|&ancestor| roots.contains(ancestor))
                        .unwrap_or(enemy);
                    commands.entity(root).despawn();
                }
            }
            commands.entity(entity).despawn();
        }
    }
}
