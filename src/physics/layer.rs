use avian3d::prelude::{CollisionLayers, PhysicsLayer};

#[derive(Debug, PhysicsLayer, Default)]
pub enum CollisionLayer {
    #[default]
    Default,
    Obstacle,
    Player,
    Prop,
    Npc,
}

impl CollisionLayer {
    pub fn get_collision_layers(&self) -> CollisionLayers {
        match self {
            CollisionLayer::Default => {
                CollisionLayers::new(CollisionLayer::Default, [CollisionLayer::Default])
            }
            CollisionLayer::Obstacle => CollisionLayers::new(
                CollisionLayer::Obstacle,
                [
                    CollisionLayer::Player,
                    CollisionLayer::Prop,
                    CollisionLayer::Npc,
                ],
            ),
            CollisionLayer::Player => {
                CollisionLayers::new(CollisionLayer::Player, [CollisionLayer::Obstacle])
            }
            CollisionLayer::Prop => CollisionLayers::new(
                CollisionLayer::Prop,
                [
                    CollisionLayer::Obstacle,
                    CollisionLayer::Prop,
                    CollisionLayer::Npc,
                ],
            ),
            CollisionLayer::Npc => CollisionLayers::new(
                CollisionLayer::Npc,
                [
                    CollisionLayer::Obstacle,
                    CollisionLayer::Prop,
                    CollisionLayer::Npc,
                ],
            ),
        }
    }
}
