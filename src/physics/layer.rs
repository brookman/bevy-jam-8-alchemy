use avian3d::prelude::PhysicsLayer;

#[derive(Debug, PhysicsLayer, Default)]
pub enum CollisionLayer {
    #[default]
    Default,
    Obstacle,
    Player,
    Prop,
    None,
}