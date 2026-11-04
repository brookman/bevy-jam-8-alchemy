use crate::loading::AppState;
use avian3d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_damping.run_if(in_state(AppState::Running)));
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[require(AngularDamping, CollidingEntities)]
#[type_path = "game"]
pub struct AdaptiveAngularDamping {
    pub default: f32,
    pub colliding: f32,
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
