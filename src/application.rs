mod actor;
mod shared;

use crate::application::actor::actor_application_service::ActorApplicationService;
use crate::domain::{
    models::{
        actor::{
            actor_id::ActorId, actor_repository::ActorRepository, direction::Direction,
            speed::Speed, Actor,
        },
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

pub struct RpgGameService<T>
where
    T: ActorRepository,
{
    actor_application_service: ActorApplicationService<T>,
}

impl<T> GameService for RpgGameService<T>
where
    T: ActorRepository,
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

impl<T> RpgGameService<T>
where
    T: ActorRepository,
{
    pub fn new() -> Self {
        let actor_repository_rc = {
            let actor_repository_rc = Rc::new(T::new());
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
            ActorApplicationService::new(ActorId(0), actor_service, actor_repository_rc);
        Self {
            actor_application_service,
        }
    }
}

impl<T> RpgGameService<T>
where
    T: ActorRepository,
{
    fn actor_application_service(&self) -> &ActorApplicationService<T> {
        &self.actor_application_service
    }
}
