mod application;
mod domain;
mod in_memory;

use crate::{
    application::rpg_game_state::RpgGameState, domain::models::shared::game_point::GamePoint,
};
use kurenai::{
    canvas::Canvas,
    game_loop::GameLoop,
    image::{image_id::ImageId, image_repository::ImageRepository, Image},
    point::{Dot, Point},
};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::console;

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
