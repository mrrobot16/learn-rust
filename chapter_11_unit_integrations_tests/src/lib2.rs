fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn add_two(number: u32) -> u32 {
    add(number, 2)
}

fn greeting(greet: &str) -> String {
    String::from("Hello!")
    // format!("Hi, {}", greet)
}
#[cfg(test)]
mod tests {
    use super::{add_two, greeting};
    
    #[test]
    fn it_adds_up() {
        assert_eq!(add_two(2), 4);
    }
    
    #[test]
    fn greetting_contains_greet() {
        let result = greeting("John");
        assert!(result.contains("John"), "Greeting did not contain name, value was `{}`", result);
    }
    
}