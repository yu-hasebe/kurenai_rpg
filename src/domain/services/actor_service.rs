use crate::domain::models::{
    actor::{
        actor_id::ActorId, actor_repository::ActorRepository, direction::Direction, speed::Speed,
        Actor,
    },
    shared::game_point::GamePoint,
};
use derive_new::new;
use kurenai::{
    canvas::Canvas,
    image::image_repository::ImageRepository,
    key_event::{KeyEvent, KeyboardEvent},
    point::{Dot, Point},
    sprite::Sprite,
};
use std::rc::Rc;

#[derive(Clone, Debug, new)]
pub struct ActorService<T>
where
    T: ActorRepository,
{
    current_actor_id: ActorId,
    actor_repository: Rc<T>,
}

impl<T> ActorService<T>
where
    T: ActorRepository,
{
    pub fn key_event(&self, key_event: &KeyboardEvent) {
        let mut actor = self.actor_repository.find(self.current_actor_id()).unwrap();
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

    pub fn update(&self) {
        let mut actor = self.actor_repository.find(self.current_actor_id()).unwrap();
        if actor.is_moving() {
            actor.move_();
        }
        self.actor_repository.save(actor).unwrap();
    }

    pub fn draw(&self, image_repository: &ImageRepository<GamePoint<Dot>>, canvas: &Canvas) {
        canvas.clear_rect(GamePoint::new(0, 0), GamePoint::new(480, 480));
        let actor = self.actor_repository.find(self.current_actor_id()).unwrap();
        actor.draw(image_repository, canvas, *actor.at()).unwrap();
    }
}

impl<T> ActorService<T>
where
    T: ActorRepository,
{
    fn current_actor_id(&self) -> &ActorId {
        &self.current_actor_id
    }
}
