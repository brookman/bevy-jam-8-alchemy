use crate::ahoy::scene::SceneAssets;
use crate::loading::AppLoadingState;
use crate::physics::collider::InitialTransform;
use avian3d::prelude::{Collisions, ComputedMass, Sleeping};
use bevy::app::{App, Update};
use bevy::log::info;
use bevy::prelude::*;
use bevy_ahoy::pickup::prelude::*;
use std::time::Duration;

pub fn plugin(app: &mut App) {
    app.insert_resource(FrameCounter(0)).add_systems(
        Update,
        (spawn_prop, prop_thrown, prop_dropped, explode).run_if(in_state(AppLoadingState::Loaded)),
    );
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[type_path = "game"]
struct Potion;

#[derive(Resource)]
struct FrameCounter(usize);

#[derive(Component, Default)]
struct ExplodeOnImpact {
    forces: [f32; 3],
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "game"]
struct PotionSpawner {
    spawn_timer: Timer,
}

impl Default for PotionSpawner {
    fn default() -> Self {
        Self {
            spawn_timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
        }
    }
}

fn spawn_prop(
    spawners: Query<(&mut PotionSpawner, &GlobalTransform)>,
    assets: Res<SceneAssets>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut spawner, transform) in spawners {
        spawner.spawn_timer.tick(time.delta());
        if spawner.spawn_timer.just_finished() {
            commands.spawn((
                WorldAssetRoot(assets.potion_scene.clone()),
                InitialTransform(transform.clone().compute_transform()),
            ));
        }
    }
}

fn prop_thrown(
    mut message_reader: MessageReader<PropThrown>,
    potion: Query<(), With<Potion>>,

    mut commands: Commands,
) {
    for thrown in message_reader.read() {
        if potion.get(thrown.prop).is_ok() {
            info!("Potion thrown {:?}", thrown.prop);
            commands
                .entity(thrown.prop)
                .insert(ExplodeOnImpact::default());
        }
    }
}

fn prop_dropped(
    mut message_reader: MessageReader<PropDropped>,
    potion: Query<(), With<Potion>>,
    mut commands: Commands,
) {
    for thrown in message_reader.read() {
        if potion.get(thrown.prop).is_ok() {
            info!("Potion dropped {:?}", thrown.prop);
            commands
                .entity(thrown.prop)
                .insert(ExplodeOnImpact::default());
        }
    }
}

const CRITICAL_FORCE: f32 = 1.1;

fn explode(
    entities: Query<(Entity, &ComputedMass), With<ExplodeOnImpact>>,
    mut explode: Query<&mut ExplodeOnImpact>,
    collisions: Collisions,
    sleeping: Query<(), With<Sleeping>>,
    mut frame_counter: ResMut<FrameCounter>,
    mut commands: Commands,
) {
    let current_frame = frame_counter.0;
    frame_counter.0 += 1;

    for (entity, mass) in entities {
        let mut total_impulse = 0.0;

        for contact_pair in collisions.collisions_with(entity) {
            total_impulse += contact_pair.total_normal_impulse_magnitude() * mass.inverse();
        }

        let mut forces = explode.get_mut(entity).unwrap().forces;
        forces[current_frame % 3] = total_impulse;
        let average = forces.iter().sum::<f32>() / 3.0;

        if average > CRITICAL_FORCE {
            commands.entity(entity).despawn();
            continue;
        }

        if sleeping.get(entity).is_ok() {
            commands.entity(entity).remove::<ExplodeOnImpact>();
        }
    }
}
