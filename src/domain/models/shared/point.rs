use std::{
    marker::PhantomData,
    ops::{Add, Sub},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point<T> {
    unit: PhantomData<T>,
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Dot {}

impl<T> Point<T> {
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            unit: PhantomData::<T>,
            x,
            y,
        }
    }

    pub fn x(&self) -> &i64 {
        &self.x
    }

    pub fn y(&self) -> &i64 {
        &self.y
    }
}

impl<T> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            unit: self.unit,
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            unit: self.unit,
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
