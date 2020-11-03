pub mod actor_id;
pub mod actor_repository;
pub mod direction;
pub mod speed;

use crate::domain::models::{
    actor::{actor_id::ActorId, direction::Direction, speed::Speed},
    shared::{
        canvas::{TILE_HEIGHT, TILE_WIDTH},
        key_code::KeyCode,
        point::{Dot, Point},
    },
};
use derive_new::new;
use kurenai::image::ImageId;

#[derive(Clone, Debug, new)]
pub struct Actor {
    id: ActorId,
    image_id: ImageId,
    size: Point<Dot>,
    at: Point<Dot>,
    direction: Direction,
    speed: Speed,
}

impl PartialEq for Actor {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Actor {}

impl Actor {
    pub fn move_from_staying(&mut self, key_code: &KeyCode) {
        if self.is_staying() {
            match key_code {
                KeyCode::ArrowLeft => self.turn(Direction::Left),
                KeyCode::ArrowUp => self.turn(Direction::Up),
                KeyCode::ArrowRight => self.turn(Direction::Right),
                KeyCode::ArrowDown => self.turn(Direction::Down),
                _ => panic!("invalid key_code"),
            }
            self.move_();
        }
    }

    pub fn move_to_staying(&mut self) {
        if self.is_moving() {
            self.move_();
        }
    }
}

impl Actor {
    pub fn id(&self) -> &ActorId {
        &self.id
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
    fn move_(&mut self) {
        let direction = *self.direction();
        let speed = *self.speed();
        let at_mut = self.at_mut();
        *at_mut = *at_mut
            + match direction {
                Direction::Left => Point::new(-(speed as i64), 0),
                Direction::Up => Point::new(0, -(speed as i64)),
                Direction::Right => Point::new(speed as i64, 0),
                Direction::Down => Point::new(0, speed as i64),
            };
    }

    fn turn(&mut self, direction: Direction) {
        if self.is_moving() {
            panic!("Call this method when the actor is staying.");
        }
        self.set_direction(direction);
    }

    fn is_staying(&self) -> bool {
        self.at().x() % TILE_WIDTH == 0 && self.at().y() % TILE_HEIGHT == 0
    }

    fn is_moving(&self) -> bool {
        !self.is_staying()
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
            Point::new(TILE_WIDTH, TILE_HEIGHT),
            Point::new(0, 0),
            Direction::Left,
            Speed::Normal,
        );
        let actor2 = Actor::new(
            ActorId(0),
            ImageId(1),
            Point::new(TILE_WIDTH, TILE_HEIGHT),
            Point::new(4, 4),
            Direction::Up,
            Speed::Normal,
        );
        let actor3 = Actor::new(
            ActorId(1),
            ImageId(0),
            Point::new(TILE_WIDTH, TILE_HEIGHT),
            Point::new(0, 0),
            Direction::Left,
            Speed::Normal,
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
            Point::new(TILE_WIDTH, TILE_HEIGHT),
            Point::new(0, 0),
            Direction::Right,
            Speed::Normal,
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
            assert_eq!(&Point::new(TILE_WIDTH, speed * i), actor.at());
            assert_eq!(&Direction::Down, actor.direction());
        }

        actor.turn(Direction::Left);
        assert_eq!(&Direction::Left, actor.direction());
        for i in 1..=8 {
            actor.move_();
            assert_eq!(&Point::new(TILE_WIDTH - speed * i, TILE_HEIGHT), actor.at());
            assert_eq!(&Direction::Left, actor.direction());
        }

        actor.turn(Direction::Up);
        assert_eq!(&Direction::Up, actor.direction());
        for i in 1..=8 {
            actor.move_();
            assert_eq!(&Point::new(0, TILE_HEIGHT - speed * i), actor.at());
            assert_eq!(&Direction::Up, actor.direction());
        }
    }

    #[test]
    #[should_panic]
    fn test_turn_should_panic() {
        let mut actor = Actor::new(
            ActorId(0),
            ImageId(0),
            Point::new(TILE_WIDTH, TILE_HEIGHT),
            Point::new(1, 1),
            Direction::Left,
            Speed::Normal,
        );
        actor.turn(Direction::Up);
    }
}
