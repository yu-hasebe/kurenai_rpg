mod application;
mod domain;
mod in_memory;

use crate::application::rpg_game_service::RpgGameService;
use kurenai::{
    canvas::{Canvas, CanvasId, CanvasRepository},
    game_loop::GameLoop,
    image::{Image, ImageId, ImageRepository},
};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&JsValue::from_str("Hello world!"));

    let rpg_game_service = RpgGameService::new();

    let canvas_repository = {
        let new_html_canvas_element = Canvas::get_html_canvas_element_by_id("main-canvas").unwrap();
        let canvas_repository = CanvasRepository::new();
        canvas_repository.save(Canvas::new(CanvasId(0), new_html_canvas_element).unwrap());
        canvas_repository
    };

    let image_repository = {
        let bytes = include_bytes!("./sample.gif");
        let new_html_image_element_rc =
            Rc::new(Image::create_new_html_image_element(bytes, "gif").unwrap());
        let image_repository = ImageRepository::new();
        image_repository.save(Image::new(
            ImageId(0),
            new_html_image_element_rc.clone(),
            64,
            96,
            32,
            32,
        ));
        image_repository.save(Image::new(
            ImageId(1),
            new_html_image_element_rc.clone(),
            64,
            64,
            32,
            32,
        ));
        image_repository
    };
    GameLoop::run(rpg_game_service, image_repository, canvas_repository).unwrap();

    Ok(())
}
