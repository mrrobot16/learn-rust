
struct Shape;

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
#[cfg(test)]
mod tests {
    // this makes everything in the outer scope valid within the tests module.
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    
    #[test]
    fn exploration() {
        let result = 3 + 1 + 16;
        assert_eq!(result, 20);
        println!("Explore The world!");
    }

    #[test]
    fn fail() {
        panic!("This is intended to fail");
    }
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 20,
            height: 15,
        };
        
        let smaller = Rectangle {
            width: 19,
            height: 14,
        };
        
        assert!(larger.can_hold(&smaller));
    }
    
    #[test]
        fn smaller_cannot_hold_larger() {
            let larger = Rectangle {
                width: 8,
                height: 7,
            };
            
            let smaller = Rectangle {
                width: 5,
                height: 1,
            };

            assert!(!smaller.can_hold(&larger));
        }
}

