struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 {
            
            panic!("Guess value must be more than or equal to 1, we got {}", value);
        }
        if value > 100 {
            panic!("Guess value must be less than or equal to 100, we got {}", value);
        }
        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Guess};
    
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        let number = 200;
        Guess::new(number);
    }
    
    #[test]
    fn less_than_100() {
        let number = 100;
        Guess::new(number);
    }
    
    #[test]
    #[should_panic(expected = "Guess value must be more than or equal to 1")]
    fn greater_than_1() {
        let number = 0;
        Guess::new(number);
    }
    
    #[test]
    fn less_than_1() {
        let number = 1;
        Guess::new(number);
    }
}