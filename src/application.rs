mod actor;
mod shared;

use crate::application::actor::actor_application_service::ActorApplicationService;
use crate::domain::{
    models::{
        actor::{
            actor_id::ActorId, actor_repository::ActorRepository, direction::Direction,
            speed::Speed, Actor,
        },
        scene::{scene_repository::SceneRepository, Scene},
        shared::point::Point,
    },
    services::actor_service::ActorService,
};
use kurenai::{
    canvas::CanvasRepository,
    image::{ImageId, ImageRepository},
    key_event::KeyEvent,
    traits::game_service::GameService,
};
use std::rc::Rc;

pub struct RpgGameService<SR, AR>
where
    SR: SceneRepository,
    AR: ActorRepository,
{
    actor_application_service: ActorApplicationService<SR, AR>,
}

impl<SR, AR> GameService for RpgGameService<SR, AR>
where
    SR: SceneRepository,
    AR: ActorRepository,
{
    fn key_event(&self, key_event: &KeyEvent) {
        self.actor_application_service().key_event(key_event);
    }

    fn update(&self) {
        self.actor_application_service().update();
    }

    fn draw(&self, image_repository: &ImageRepository, canvas_repository: &CanvasRepository) {
        self.actor_application_service()
            .draw(image_repository, canvas_repository);
    }
}

impl<SR, AR> RpgGameService<SR, AR>
where
    SR: SceneRepository,
    AR: ActorRepository,
{
    pub fn new() -> Self {
        let scene_repository_rc = Rc::new(SR::new(Scene::new(ActorId(0))));
        let actor_repository_rc = {
            let actor_repository_rc = Rc::new(AR::new());
            let actor = Actor::new(
                ActorId(0),
                ImageId(0),
                Point::new(32, 32),
                Point::new(0, 0),
                Direction::Down,
                Speed(4),
            );
            actor_repository_rc.save(actor).unwrap();
            actor_repository_rc
        };
        let actor_service = ActorService::new(actor_repository_rc.clone());
        let actor_application_service =
            ActorApplicationService::new(actor_service, scene_repository_rc, actor_repository_rc);
        Self {
            actor_application_service,
        }
    }
}

impl<SR, AR> RpgGameService<SR, AR>
where
    SR: SceneRepository,
    AR: ActorRepository,
{
    fn actor_application_service(&self) -> &ActorApplicationService<SR, AR> {
        &self.actor_application_service
    }
}
