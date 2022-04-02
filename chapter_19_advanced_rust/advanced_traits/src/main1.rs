use advanced_traits::{Point, Millimeters, Meters};

fn main() {
    println!("Hello, world of Advanced Trait!");
    
    let point_input1 = Point {
        x: 5, 
        y: 1,
    };
    
    let point_input2 = Point {
        x: 5,
        y: 4,
    };
    
    let point_output = Point {
        x: 10,
        y: 5,
    };
    
    let point_input = point_input1 + point_input2;
    
    assert_eq!(point_input, point_output);
    
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    
    // 19.15
    let millimeters = Millimeters(1500.0);
    let meters = Meters(2.5);
    println!("millimeters: {:#?}", millimeters);
    println!("meters: {:#?}", meters);
    let millimeters_plus_meters = millimeters + meters;
    println!("millimeters + meters: {:?}", millimeters_plus_meters);
}
