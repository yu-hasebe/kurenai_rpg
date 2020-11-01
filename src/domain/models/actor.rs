pub mod actor_id;
pub mod direction;
pub mod speed;

use crate::{
    domain::models::actor::{actor_id::ActorId, direction::Direction, speed::Speed},
    kurenai_shared::game_point::GamePoint,
};
use derive_new::new;
use kurenai::{
    image::image_id::ImageId,
    point::{Dot, Point},
    sprite::Sprite,
};

#[derive(Clone, Debug, new)]
pub struct Actor {
    actor_id: ActorId,
    image_id: ImageId,
    size: GamePoint<Dot>,
    at: GamePoint<Dot>,
    direction: Direction,
    speed: Speed,
}

impl PartialEq for Actor {
    fn eq(&self, other: &Self) -> bool {
        self.actor_id() == other.actor_id()
    }
}

impl Eq for Actor {}

impl Sprite<GamePoint<Dot>> for Actor {
    fn image_id(&self) -> &ImageId {
        &self.image_id
    }

    fn size(&self) -> &GamePoint<Dot> {
        &self.size
    }
}

impl Actor {
    pub fn at(&self) -> &GamePoint<Dot> {
        &self.at
    }
}

impl Actor {
    fn actor_id(&self) -> &ActorId {
        &self.actor_id
    }
}
