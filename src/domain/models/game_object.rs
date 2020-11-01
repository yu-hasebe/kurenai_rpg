use crate::kurenai_shared::game_point::GamePoint;
use kurenai::{image::image_id::ImageId, point::Dot, sprite::Sprite};

#[derive(Clone, Debug)]
pub struct GameObject {
    image_id: ImageId,
    size: GamePoint<Dot>,
    pub at: GamePoint<Dot>,
}

impl GameObject {
    pub fn new(image_id: ImageId, size: GamePoint<Dot>, at: GamePoint<Dot>) -> Self {
        Self { image_id, size, at }
    }
}

impl Sprite<GamePoint<Dot>> for GameObject {
    fn image_id(&self) -> &ImageId {
        &self.image_id
    }

    fn size(&self) -> &GamePoint<Dot> {
        &self.size
    }
}

impl GameObject {
    pub fn at(&self) -> &GamePoint<Dot> {
        &self.at
    }
}
