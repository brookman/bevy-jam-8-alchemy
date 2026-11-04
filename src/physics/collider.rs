use crate::loading::AppState;
use crate::physics::damping::AdaptiveAngularDamping;
pub(crate) use crate::physics::layer::CollisionLayer;
use avian3d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
        (setup_obstacles, setup_props)
            .after(TransformSystems::Propagate)
            .run_if(in_state(AppState::Running)),
    );
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[type_path = "game"]
struct Obstacle;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[type_path = "game"]
struct Prop;

#[derive(Component, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "game"]
enum ColliderShape {
    Cuboid { x: f32, y: f32, z: f32 },
    Sphere { radius: f32 },
    Cylinder { radius: f32, length: f32 },
    Cone { radius: f32, length: f32 },
}

impl Default for ColliderShape {
    fn default() -> Self {
        Self::Cuboid {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
}

impl ColliderShape {
    fn create_collider(&self) -> Collider {
        match self {
            ColliderShape::Cuboid { x, y, z } => Collider::cuboid(*x, *y, *z),
            ColliderShape::Sphere { radius } => Collider::sphere(*radius),
            ColliderShape::Cylinder { radius, length } => Collider::cylinder(*radius, *length),
            ColliderShape::Cone { radius, length } => Collider::cone(*radius, *length),
        }
    }
}

fn setup_obstacles(
    mut commands: Commands,
    bodies: Query<(Entity, &GlobalTransform), (With<Obstacle>, Without<RigidBody>)>,
    colliders: Query<(&GlobalTransform, &ColliderShape)>,
    children: Query<&Children>,
) {
    for (entity, body_transform) in &bodies {
        commands.entity(entity).insert((
            RigidBody::Static,
            CollisionLayers::new(
                CollisionLayer::Obstacle,
                [CollisionLayer::Player, CollisionLayer::Prop],
            ),
            build_compound_collider(entity, body_transform, colliders, children),
        ));
    }
}

fn setup_props(
    mut commands: Commands,
    bodies: Query<(Entity, &GlobalTransform), (With<Prop>, Without<RigidBody>)>,
    colliders: Query<(&GlobalTransform, &ColliderShape)>,
    children: Query<&Children>,
) {
    for (entity, body_transform) in &bodies {
        commands.entity(entity).insert((
            RigidBody::Dynamic,
            CollisionLayers::new(
                CollisionLayer::Prop,
                [CollisionLayer::Obstacle, CollisionLayer::Prop],
            ),
            build_compound_collider(entity, body_transform, colliders, children),
            ColliderDensity(300.0),
            AdaptiveAngularDamping {
                default: 0.1,
                colliding: 0.5,
            },
        ));
    }
}

fn build_compound_collider(
    entity: Entity,
    body_transform: &GlobalTransform,
    colliders: Query<(&GlobalTransform, &ColliderShape)>,
    children: Query<&Children>,
) -> Collider {
    let mut shapes = Vec::new();
    if let Ok((_, shape)) = colliders.get(entity) {
        shapes.push((Vec3::ZERO, Quat::IDENTITY, shape.create_collider()))
    }
    for descendant in children.iter_descendants(entity) {
        if let Ok((collider_transform, shape)) = colliders.get(descendant) {
            let relative = body_transform.affine().inverse() * collider_transform.affine();

            let (_, rotation, translation) = relative.to_scale_rotation_translation();

            shapes.push((translation, rotation, shape.create_collider()));
        }
    }
    Collider::compound(shapes)
}
