use crate::{
    application::shared,
    domain::{
        models::actor::{actor_id::ActorId, actor_repository::ActorRepository},
        services::actor_service::ActorService,
    },
};
use derive_new::new;
use kurenai::{
    canvas::{CanvasId, CanvasRepository},
    image::ImageRepository,
    key_event::KeyEvent,
};
use std::rc::Rc;

#[derive(Clone, Debug, new)]
pub struct ActorApplicationService<T>
where
    T: ActorRepository,
{
    current_actor_id: ActorId,
    actor_service: ActorService<T>,
    actor_repository: Rc<T>,
}

impl<T> ActorApplicationService<T>
where
    T: ActorRepository,
{
    pub fn key_event(&self, key_event: &KeyEvent) {
        let mut actor = self
            .actor_repository()
            .find(self.current_actor_id())
            .unwrap();
        if let Some(key_code) = shared::key_event_arrow_to_key_code(key_event) {
            actor.move_from_staying(&key_code);
        }
        self.actor_repository.save(actor).unwrap();
    }

    pub fn update(&self) {
        let mut actor = self
            .actor_repository()
            .find(self.current_actor_id())
            .unwrap();
        actor.move_to_staying();
        self.actor_repository.save(actor).unwrap();
    }

    pub fn draw(&self, image_repository: &ImageRepository, canvas_repository: &CanvasRepository) {
        let canvas = canvas_repository.find(&CanvasId(0)).unwrap();
        canvas.context().clear_rect(0.0, 0.0, 480.0, 480.0);
        let actor = self
            .actor_repository()
            .find(self.current_actor_id())
            .unwrap();
        let image = image_repository.find(actor.image_id()).unwrap();
        canvas
            .context()
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image.html_image_element(),
                *image.begin_dot_x() as f64,
                *image.begin_dot_y() as f64,
                *image.width() as f64,
                *image.height() as f64,
                *actor.at().x() as f64,
                *actor.at().y() as f64,
                *actor.size().x() as f64,
                *actor.size().y() as f64,
            )
            .unwrap();
    }
}

impl<T> ActorApplicationService<T>
where
    T: ActorRepository,
{
    fn current_actor_id(&self) -> &ActorId {
        &self.current_actor_id
    }

    fn actor_repository(&self) -> Rc<T> {
        self.actor_repository.clone()
    }
}
