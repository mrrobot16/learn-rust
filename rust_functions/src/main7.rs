fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    
    let rectangle1 = Rectangle {
        width: 10,
        height: 40
    };
    
    let rectangle2 = Rectangle {
        width: 60,
        height: 45
    };
    
    let rectangle3 = Rectangle {
        width: 31,
        height: 49,
    };
    
    if rectangle.width() {
        println!("The rectangle has a nonzero width; it is {}", rectangle.width);
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );
    
    println!("can rectangle hold rectangle1? {}", rectangle.can_hold(&rectangle1));
    println!("can rectangle hold rectangle2? {}", rectangle.can_hold(&rectangle2));
    println!("can rectangle hold rectangle3? {}", rectangle3.can_hold(&rectangle));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}