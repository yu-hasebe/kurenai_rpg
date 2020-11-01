use crate::{
    domain::{
        models::{
            actor::{
                actor_id::ActorId, actor_repository::ActorRepository, direction::Direction,
                speed::Speed, Actor,
            },
            shared::game_point::GamePoint,
        },
        services::actor_service::ActorService,
    },
    in_memory::actor::in_memory_actor_repository::InMemoryActorRepository,
};
use kurenai::{
    canvas::Canvas,
    game_state::GameState,
    image::{image_id::ImageId, image_repository::ImageRepository},
    key_event::KeyboardEvent,
    point::{Dot, Point},
};
use std::rc::Rc;

pub struct RpgGameState<T>
where
    T: ActorRepository,
{
    actor_service: ActorService<T>,
}

impl<T> GameState<KeyboardEvent, GamePoint<Dot>> for RpgGameState<T>
where
    T: ActorRepository,
{
    fn key_event(&mut self, key_event: &KeyboardEvent) {
        self.actor_service.key_event(key_event);
    }

    fn update(&mut self) {
        self.actor_service.update();
    }

    fn draw(&self, image_repository: &ImageRepository<GamePoint<Dot>>, canvas: &Canvas) {
        self.actor_service.draw(image_repository, canvas);
    }
}

impl RpgGameState<InMemoryActorRepository> {
    pub fn new() -> Self {
        let actor_repository_rc = Rc::new(InMemoryActorRepository::new());
        let actor = Actor::new(
            ActorId(0),
            ImageId(0),
            GamePoint::new(32, 32),
            GamePoint::new(0, 0),
            Direction::Down,
            Speed(4),
        );
        actor_repository_rc.save(actor).unwrap();
        let actor_service = ActorService::new(ActorId(0), actor_repository_rc);
        Self { actor_service }
    }
}
