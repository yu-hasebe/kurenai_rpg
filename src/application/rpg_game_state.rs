use crate::{
    domain::models::{
        actor::{
            actor_id::ActorId, actor_repository::ActorRepository, direction::Direction,
            speed::Speed, Actor,
        },
        shared::game_point::GamePoint,
    },
    in_memory::actor::in_memory_actor_repository::InMemoryActorRepository,
};
use kurenai::{
    canvas::Canvas,
    game_state::GameState,
    image::{image_id::ImageId, image_repository::ImageRepository},
    key_event::{KeyEvent, KeyboardEvent},
    point::{Dot, Point},
    sprite::Sprite,
};
use std::rc::Rc;

pub struct RpgGameState<T>
where
    T: ActorRepository,
{
    actor_repository: Rc<T>,
}

impl<T> GameState<KeyboardEvent, GamePoint<Dot>> for RpgGameState<T>
where
    T: ActorRepository,
{
    fn key_event(&mut self, key_event: &KeyboardEvent) {
        let mut actor = self.actor_repository.find(&ActorId(0)).unwrap();
        if actor.is_staying() {
            if key_event.arrow_left() {
                actor.turn(Direction::Left);
                actor.move_();
            } else if key_event.arrow_down() {
                actor.turn(Direction::Down);
                actor.move_();
            } else if key_event.arrow_right() {
                actor.turn(Direction::Right);
                actor.move_();
            } else if key_event.arrow_up() {
                actor.turn(Direction::Up);
                actor.move_();
            }
        }
        self.actor_repository.save(actor).unwrap();
    }

    fn update(&mut self) {
        let mut actor = self.actor_repository.find(&ActorId(0)).unwrap();
        if actor.is_moving() {
            actor.move_();
        }
        self.actor_repository.save(actor).unwrap();
    }

    fn draw(&self, image_repository: &ImageRepository<GamePoint<Dot>>, canvas: &Canvas) {
        let actor = self.actor_repository.find(&ActorId(0)).unwrap();
        actor.draw(image_repository, canvas, *actor.at()).unwrap();
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
        Self {
            actor_repository: actor_repository_rc,
        }
    }
}
