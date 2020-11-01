use crate::{domain::models::game_object::GameObject, kurenai_shared::game_point::GamePoint};
use kurenai::{
    canvas::Canvas,
    game_state::GameState,
    image::{image_id::ImageId, image_repository::ImageRepository},
    key_event::{KeyEvent, KeyboardEvent},
    point::{Dot, Point},
    sprite::Sprite,
};

pub struct RpgGameState {
    data1: GameObject,
    data2: GameObject,
}

impl GameState<KeyboardEvent, GamePoint<Dot>> for RpgGameState {
    fn key_event(&mut self, key_event: &KeyboardEvent) {
        if key_event.enter() {
            self.data1.at = GamePoint::new(0, 0);
            self.data2.at = GamePoint::new(0, 0);
        }
    }

    fn update(&mut self) {
        self.data1.at = self.data1.at.clone() + GamePoint::new(1, 0);
        self.data2.at = self.data2.at.clone() + GamePoint::new(0, 1);
    }

    fn draw(&self, image_repository: &ImageRepository<GamePoint<Dot>>, canvas: &Canvas) {
        self.data1
            .draw(image_repository, canvas, *self.data1.at())
            .unwrap();
        self.data2
            .draw(image_repository, canvas, *self.data2.at())
            .unwrap();
    }
}

impl RpgGameState {
    pub fn new() -> Self {
        let data1 = GameObject::new(ImageId(0), GamePoint::new(32, 32), GamePoint::new(0, 0));
        let data2 = GameObject::new(ImageId(1), GamePoint::new(32, 32), GamePoint::new(0, 0));
        Self { data1, data2 }
    }
}
