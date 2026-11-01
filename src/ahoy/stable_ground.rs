use avian3d::parry::glamx::Vec3;
use avian3d::prelude::LinearVelocity;
use bevy::app::{App, Update};
use bevy::prelude::{
    Component, IntoScheduleConfigs, Query, Reflect, Res, Time, Timer, TimerMode, Transform,
};
use bevy::time::common_conditions::on_timer;
use bevy_ahoy::{CharacterController, CharacterControllerState};
use std::collections::VecDeque;
use std::time::Duration;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            calculate_stable_ground.run_if(on_timer(Duration::from_secs(1))),
            apply_last_stable_ground.after(calculate_stable_ground),
        ),
    );
}

#[derive(Component, Reflect, Clone)]
pub struct StableGround {
    pub previous: VecDeque<Vec3>,
    fall_timer: Timer,
}
impl Default for StableGround {
    fn default() -> Self {
        Self {
            previous: VecDeque::default(),
            fall_timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
        }
    }
}

pub(crate) fn calculate_stable_ground(
    mut kccs: Query<(&Transform, &CharacterControllerState, &mut StableGround)>,
) {
    for (transform, state, mut stable_ground) in &mut kccs {
        let Some(ground) = state.grounded else {
            continue;
        };

        let up_diff = (1. - ground.normal1.y).abs();

        // If we don't compare to EPSILON, Vec3::y will *almost* always be 0.9...
        if up_diff <= f32::EPSILON {
            stable_ground.previous.push_front(transform.translation);

            // Used to ensure that player doesn't get stuck in infinite loop if the most recent
            // stable ground wasn't so stable.
            while stable_ground.previous.len() > 5 {
                stable_ground.previous.pop_back();
            }
        }
    }
}

pub(crate) fn apply_last_stable_ground(
    mut kccs: Query<(
        &mut Transform,
        &LinearVelocity,
        &CharacterController,
        &mut StableGround,
    )>,
    time: Res<Time>,
) {
    for (mut transform, velocity, controller, mut stable_ground) in &mut kccs {
        let speed_diff = 1. - (velocity.0.y.abs() / controller.max_speed);

        // Terminal velocity will take quite a while to reach exactly 100., so we compare to 0.01
        // to ensure that it doesn't take longer than expected
        if speed_diff <= 0.01 {
            stable_ground.fall_timer.tick(time.elapsed());
        } else {
            stable_ground.fall_timer.reset();
        }

        let max_fall_elapsed = stable_ground.fall_timer.is_finished();

        if max_fall_elapsed && let Some(last_stable_ground) = stable_ground.previous.pop_front() {
            transform.translation = last_stable_ground;
        }
    }
}
