mod kurenai_shared;
use kurenai::{
    canvas::Canvas,
    game_loop::GameLoop,
    game_state::GameState,
    image::{image_id::ImageId, image_repository::ImageRepository, Image},
    key_event::{KeyEvent, KeyboardEvent},
    point::{Dot, Point},
    sprite::Sprite,
};
use kurenai_shared::game_point::GamePoint;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::console;

struct RpgGameState {
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
    fn new() -> Self {
        let data1 = GameObject::new(ImageId(0), GamePoint::new(32, 32), GamePoint::new(0, 0));
        let data2 = GameObject::new(ImageId(1), GamePoint::new(32, 32), GamePoint::new(0, 0));
        Self { data1, data2 }
    }
}

#[derive(Clone, Debug)]
struct GameObject {
    image_id: ImageId,
    size: GamePoint<Dot>,
    at: GamePoint<Dot>,
}

impl GameObject {
    fn new(image_id: ImageId, size: GamePoint<Dot>, at: GamePoint<Dot>) -> Self {
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
    fn at(&self) -> &GamePoint<Dot> {
        &self.at
    }
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console::log_1(&JsValue::from_str("Hello world!"));

    let rpg_game_state = RpgGameState::new();
    let canvas = Canvas::new("main-canvas", GamePoint::new(480, 480), "game-container").unwrap();
    let image_repository = {
        let bytes = include_bytes!("./sample.gif");
        let new_html_image_element_rc =
            Rc::new(Image::<GamePoint<Dot>>::create_new_html_image_element(bytes, "gif").unwrap());
        let image_repository = ImageRepository::new();
        image_repository
            .save(Image::new(
                ImageId(0),
                new_html_image_element_rc.clone(),
                GamePoint::new(64, 96),
                GamePoint::new(32, 32),
            ))
            .unwrap();
        image_repository
            .save(Image::new(
                ImageId(1),
                new_html_image_element_rc.clone(),
                GamePoint::new(64, 64),
                GamePoint::new(32, 32),
            ))
            .unwrap();
        image_repository
    };
    GameLoop::run(rpg_game_state, image_repository, canvas).unwrap();

    Ok(())
}
