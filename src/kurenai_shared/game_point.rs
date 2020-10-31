use kurenai::point::Point;
use std::{
    marker::PhantomData,
    ops::{Add, Sub},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GamePoint<T> {
    unit: PhantomData<T>,
    x: i64,
    y: i64,
}

impl<T> Point<T> for GamePoint<T> {
    fn new(x: i64, y: i64) -> Self {
        Self {
            unit: PhantomData::<T>,
            x,
            y,
        }
    }

    fn x(&self) -> &i64 {
        &self.x
    }

    fn y(&self) -> &i64 {
        &self.y
    }
}

impl<T> Add for GamePoint<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            unit: self.unit,
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for GamePoint<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            unit: self.unit,
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
