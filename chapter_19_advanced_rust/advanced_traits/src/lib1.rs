use std::ops::Add;

// 19.15
#[derive (Debug)]
pub struct Millimeters(pub f64);

#[derive (Debug)]
pub struct Meters(pub f64);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000.0))
    }
}

// 19.14
#[derive (Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0
        }
    }

}

// 19.12
pub trait Iterator<T> {
    type Item;
    
    // fn next(&mut self) -> Option<Self::Item>;
    fn next(&mut self) -> Option<u32>;
}
// 19.13
impl<T> Iterator<T> for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}