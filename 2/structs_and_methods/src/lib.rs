use noisy_float::prelude::*;
extern crate num_traits;
use num_traits::Num;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Point <T : Num> {
    pub x: T,
    pub y: T
}

impl <T : Num> Default for Point <T> {
    fn default() -> Self {
        Self { x: T::zero(), y: T::zero() }
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Rect <T : Num> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T
}

impl <T : Num + PartialOrd + Copy> Rect <T> {
    pub fn contains(&self, p: &Point<T>) -> bool {
        p.x > self.x && p.x < self.x + self.w && p.y > self.y && p.y < self.y + self.h
    }

    pub fn area(&self) -> T {
        self.w * self.h
    }
}

impl <T : Num> Default for Rect <T> {
    fn default() -> Self {
        Self { x: T::zero(), y: T::zero(), w: T::one(), h: T::one() }
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Circle <T : Num> {
    pub x: T,
    pub y: T,
    pub r: T
}

impl <T : Num + From<f64> + PartialOrd + Copy> Circle <T> {
    pub fn contains(&self, p: &Point <T> ) -> bool {
        (p.x - self.x)*(p.x - self.x) + (p.y - self.y)*(p.y - self.y) < self.r*self.r
    }

    pub fn area(&self) -> T {
        T::from(std::f64::consts::PI) * self.r * self.r
    }
}

impl <T : Num> Default for Circle <T> {
    fn default() -> Self {
        Self { x: T::zero(), y: T::zero(), r: T::one() }
    }
}


enum Figure <T : Num> {
    Rect(Rect <T>),
    Circle(Circle <T> )
}

impl <T : Num + PartialOrd + From<f64> + Copy> Figure <T> {
    fn contains(&self, p: &Point <T>) -> bool {
        match self {
            Figure::Rect(fig) => fig.contains(p),
            Figure::Circle(fig) => fig.contains(p)
        }
    }

    fn area(&self) -> T {
        match self {
            Figure::Circle(fig) => fig.area(),
            Figure::Rect(fig) => fig.area()
        }
    }
}