use advanced_traits::{Point, OutlinePrint};

fn main() {
    println!("Hello, world of Advanced Trait!");
    
    let point = Point {
        x: 1,
        y: 3,
    };
    point.outline_print();
}
