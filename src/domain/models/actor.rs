pub mod actor_id;
pub mod actor_repository;
pub mod direction;
pub mod speed;

use crate::domain::models::{
    actor::{actor_id::ActorId, direction::Direction, speed::Speed},
    shared::point::{Dot, Point},
};
use derive_new::new;
use kurenai::image::ImageId;

#[derive(Clone, Debug, new)]
pub struct Actor {
    actor_id: ActorId,
    image_id: ImageId,
    size: Point<Dot>,
    at: Point<Dot>,
    direction: Direction,
    speed: Speed,
}

impl PartialEq for Actor {
    fn eq(&self, other: &Self) -> bool {
        self.actor_id() == other.actor_id()
    }
}

impl Eq for Actor {}

impl Actor {
    pub fn move_(&mut self) {
        let direction = *self.direction();
        let speed = self.speed().0;
        let at_mut = self.at_mut();
        *at_mut = *at_mut
            + match direction {
                Direction::Left => Point::new(-speed, 0),
                Direction::Up => Point::new(0, -speed),
                Direction::Right => Point::new(speed, 0),
                Direction::Down => Point::new(0, speed),
            };
    }

    pub fn turn(&mut self, direction: Direction) {
        if self.is_moving() {
            panic!("Call this method when the actor is staying.");
        }
        self.set_direction(direction);
    }

    pub fn is_staying(&self) -> bool {
        self.at().x() % 32 == 0 && self.at().y() % 32 == 0
    }

    pub fn is_moving(&self) -> bool {
        !self.is_staying()
    }
}

impl Actor {
    pub fn actor_id(&self) -> &ActorId {
        &self.actor_id
    }

    pub fn image_id(&self) -> &ImageId {
        &self.image_id
    }

    pub fn size(&self) -> &Point<Dot> {
        &self.size
    }

    pub fn at(&self) -> &Point<Dot> {
        &self.at
    }
}

impl Actor {
    fn at_mut(&mut self) -> &mut Point<Dot> {
        &mut self.at
    }

    fn direction(&self) -> &Direction {
        &self.direction
    }

    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn speed(&self) -> &Speed {
        &self.speed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let actor1 = Actor::new(
            ActorId(0),
            ImageId(0),
            Point::new(32, 32),
            Point::new(0, 0),
            Direction::Left,
            Speed(4),
        );
        let actor2 = Actor::new(
            ActorId(0),
            ImageId(1),
            Point::new(32, 32),
            Point::new(4, 4),
            Direction::Up,
            Speed(2),
        );
        let actor3 = Actor::new(
            ActorId(1),
            ImageId(0),
            Point::new(32, 32),
            Point::new(0, 0),
            Direction::Left,
            Speed(4),
        );
        assert_eq!(actor2, actor1);
        assert_ne!(actor3, actor1);
    }

    #[test]
    fn test_move_and_turn() {
        let speed = 4;
        let mut actor = Actor::new(
            ActorId(0),
            ImageId(0),
            Point::new(32, 32),
            Point::new(0, 0),
            Direction::Right,
            Speed(speed),
        );
        for i in 1..=8 {
            actor.move_();
            assert_eq!(&Point::new(speed * i, 0), actor.at());
            assert_eq!(&Direction::Right, actor.direction());
        }

        actor.turn(Direction::Down);
        assert_eq!(&Direction::Down, actor.direction());
        for i in 1..=8 {
            actor.move_();
            assert_eq!(&Point::new(32, speed * i), actor.at());
            assert_eq!(&Direction::Down, actor.direction());
        }

        actor.turn(Direction::Left);
        assert_eq!(&Direction::Left, actor.direction());
        for i in 1..=8 {
            actor.move_();
            assert_eq!(&Point::new(32 - speed * i, 32), actor.at());
            assert_eq!(&Direction::Left, actor.direction());
        }

        actor.turn(Direction::Up);
        assert_eq!(&Direction::Up, actor.direction());
        for i in 1..=8 {
            actor.move_();
            assert_eq!(&Point::new(0, 32 - speed * i), actor.at());
            assert_eq!(&Direction::Up, actor.direction());
        }
    }

    #[test]
    #[should_panic]
    fn test_turn_should_panic() {
        let mut actor = Actor::new(
            ActorId(0),
            ImageId(0),
            Point::new(32, 32),
            Point::new(1, 1),
            Direction::Left,
            Speed(4),
        );
        actor.turn(Direction::Up);
    }
}
