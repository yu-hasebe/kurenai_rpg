use crate::{
    domain::{
        models::{
            actor::{
                actor_id::ActorId, actor_repository::ActorRepository, direction::Direction,
                speed::Speed, Actor,
            },
            shared::point::Point,
        },
        services::actor_service::ActorService,
    },
    in_memory::actor::in_memory_actor_repository::InMemoryActorRepository,
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
    actor_service: ActorService<T>,
}

impl<T> GameService for RpgGameService<T>
where
    T: ActorRepository,
{
    fn key_event(&self, key_event: &KeyEvent) {
        self.actor_service.key_event(key_event);
    }

    fn update(&self) {
        self.actor_service.update();
    }

    fn draw(&self, image_repository: &ImageRepository, canvas_repository: &CanvasRepository) {
        self.actor_service.draw(image_repository, canvas_repository);
    }
}

impl RpgGameService<InMemoryActorRepository> {
    pub fn new() -> Self {
        let actor_repository_rc = Rc::new(InMemoryActorRepository::new());
        let actor = Actor::new(
            ActorId(0),
            ImageId(0),
            Point::new(32, 32),
            Point::new(0, 0),
            Direction::Down,
            Speed(4),
        );
        actor_repository_rc.save(actor).unwrap();
        let actor_service = ActorService::new(ActorId(0), actor_repository_rc);
        Self { actor_service }
    }
}
