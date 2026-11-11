use crate::enemy::{Enemy, EnemyAgent};
use crate::loading::AppLoadingState;
use crate::physics::damping::AdaptiveAngularDamping;
pub(crate) use crate::physics::layer::CollisionLayer;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_landmass::{
    Agent, Agent3dBundle, AgentSettings, AgentTarget3d, Archipelago3d, ArchipelagoRef3d,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
        (setup_obstacles, setup_props, setup_enemy)
            .after(TransformSystems::Propagate)
            .run_if(in_state(AppLoadingState::Loaded)),
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
    Capsule { radius: f32, length: f32 },
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
            ColliderShape::Capsule { radius, length } => Collider::capsule(*radius, *length),
        }
    }
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component, Default)]
#[type_path = "game"]
pub struct InitialTransform(pub Transform);

fn setup_obstacles(
    mut commands: Commands,
    bodies: Query<(Entity, &GlobalTransform), (With<Obstacle>, Without<RigidBody>)>,
    colliders: Query<(&GlobalTransform, &ColliderShape)>,
    children: Query<&Children>,
) {
    for (entity, body_transform) in &bodies {
        commands.entity(entity).insert((
            RigidBody::Static,
            CollisionLayer::Obstacle.get_collision_layers(),
            build_compound_collider(entity, body_transform, colliders, children),
        ));
    }
}

fn setup_props(
    mut commands: Commands,
    bodies: Query<(Entity, &GlobalTransform), (With<Prop>, Without<RigidBody>)>,
    colliders: Query<(&GlobalTransform, &ColliderShape)>,
    children: Query<&Children>,
    initial_transforms: Query<&InitialTransform>,
    hierarchy_query: Query<&ChildOf>,
) {
    for (entity, body_transform) in &bodies {
        let mut e = commands.entity(entity);
        e.insert((
            RigidBody::Dynamic,
            CollisionLayer::Prop.get_collision_layers(),
            build_compound_collider(entity, body_transform, colliders, children),
            ColliderDensity(300.0),
            AdaptiveAngularDamping {
                default: 0.1,
                colliding: 0.5,
            },
        ));

        for ancestor in hierarchy_query.iter_ancestors(entity) {
            if let Ok(initial_transform) = initial_transforms.get(ancestor) {
                e.insert(initial_transform.0.clone());
            }
        }
    }
}

fn setup_enemy(
    mut commands: Commands,
    bodies: Query<(Entity, &GlobalTransform), (With<Enemy>, Without<RigidBody>)>,
    colliders: Query<(&GlobalTransform, &ColliderShape)>,
    children: Query<&Children>,
    initial_transforms: Query<&InitialTransform>,
    hierarchy_query: Query<&ChildOf>,
    archipelago3d: Query<Entity, With<Archipelago3d>>,
) {
    let Ok(archipelago) = archipelago3d.single() else {
        return;
    };

    for (entity, body_transform) in &bodies {
        let mut enemy = commands.entity(entity);

        enemy.insert((
            RigidBody::Dynamic,
            CollisionLayer::Npc.get_collision_layers(),
            build_compound_collider(entity, body_transform, colliders, children),
            ColliderDensity(300.0),
            LockedAxes::new().lock_rotation_x().lock_rotation_z(),
        ));

        for ancestor in hierarchy_query.iter_ancestors(entity) {
            if let Ok(initial_transform) = initial_transforms.get(ancestor) {
                let desired_speed = fastrand::f32() * 3. + 2.0;
                enemy.insert((
                    initial_transform.0.clone(),
                    EnemyAgent {
                        response_time: 0.15,
                        max_acceleration: 10.0,
                    },
                    Name::new("Agent"),
                    Agent3dBundle {
                        agent: Agent::default(),
                        archipelago_ref: ArchipelagoRef3d::new(archipelago),
                        settings: AgentSettings {
                            desired_speed,
                            max_speed: desired_speed * 1.2,
                            radius: 0.2,
                        },
                    },
                    AgentTarget3d::None,
                ));

                commands.entity(ancestor).remove::<InitialTransform>();
                break;
            }
        }
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
