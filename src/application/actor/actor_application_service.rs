use crate::application::shared;
use crate::domain::{
    models::{
        actor::actor_repository::ActorRepository, map::map_repository::MapRepository,
        scene::scene_repository::SceneRepository,
    },
    services::actor_service::ActorService,
};
use derive_new::new;
use kurenai::{
    canvas::{CanvasId, CanvasRepository},
    image::ImageRepository,
    key_event::KeyEvent,
};
use std::rc::Rc;

#[derive(Clone, Debug, new)]
pub struct ActorApplicationService<SR, AR, MR>
where
    SR: SceneRepository,
    AR: ActorRepository,
    MR: MapRepository,
{
    actor_service: ActorService<SR, AR, MR>,
    scene_repository_rc: Rc<SR>,
    actor_repository_rc: Rc<AR>,
}

// TODO: You call the actor three times per frame. Use cashe or something.
impl<SR, AR, MR> ActorApplicationService<SR, AR, MR>
where
    SR: SceneRepository,
    AR: ActorRepository,
    MR: MapRepository,
{
    pub fn key_event(&self, key_event: &KeyEvent) {
        let mut actor = self
            .actor_repository_rc()
            .find(self.scene_repository_rc().find().unwrap().actor_id())
            .unwrap();
        if let Some(key_code) = shared::key_event_arrow_to_key_code(key_event) {
            actor.move_from_staying(&key_code);
        }
        self.actor_repository_rc().save(actor).unwrap();
    }

    pub fn update(&self) {
        let mut actor = self
            .actor_repository_rc()
            .find(self.scene_repository_rc().find().unwrap().actor_id())
            .unwrap();
        actor.move_to_staying();
        self.actor_repository_rc.save(actor).unwrap();
    }

    pub fn draw(&self, image_repository: &ImageRepository, canvas_repository: &CanvasRepository) {
        let canvas = canvas_repository.find(&CanvasId(0)).unwrap();
        canvas.context().clear_rect(0.0, 0.0, 480.0, 480.0);
        let actor = self
            .actor_repository_rc()
            .find(self.scene_repository_rc().find().unwrap().actor_id())
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

impl<SR, AR, MR> ActorApplicationService<SR, AR, MR>
where
    SR: SceneRepository,
    AR: ActorRepository,
    MR: MapRepository,
{
    fn scene_repository_rc(&self) -> Rc<SR> {
        self.scene_repository_rc.clone()
    }

    fn actor_repository_rc(&self) -> Rc<AR> {
        self.actor_repository_rc.clone()
    }
}
