fn internal_add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn add_two(number: i32) -> i32 {
    internal_add(number, 2)
}

pub fn add_three(number: i32) -> i32 {
    internal_add(number, 3)
}

#[cfg(test)]
mod tests {

    use super::{add_two, internal_add, setup};
    
    #[test]
    fn it_adds_up() {
        setup();
        assert_eq!(4, add_two(2));
        assert_eq!(4, internal_add(2, 2));
    }
    
    #[test]
    #[should_panic]
    fn it_does_not_add_up(){
        assert_eq!(4, add_two(2));
        assert_eq!(4, internal_add(2, 3));
    }
}