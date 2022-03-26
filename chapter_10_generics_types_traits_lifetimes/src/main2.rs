mod lib;

// pub use crate::front_of_house::hosting;
// pub use crate::lib::{Summary, Tweet};
pub use crate::lib::{Summary, Tweet};

fn main() {
    println!("Hello, world of generics!");
    let point_1 = Point { x: 5, y: 10.4 };
    let point_2 = Point { x: "Hello", y: "c" };
    let point_3 = point_1.mixup(point_2);
    
    println!("point_3.x = {}, point_3.y = {}", point_3.x, point_3.y);
}

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point { 
            x: self.x,
            y: other.y,
        }
    }
}