use advanced_traits::{Wrapper};

fn main() {
    println!("Hello, world of Advanced Trait!");
    
    // 19.23
    let vector = vec![String::from("hello"), String::from("world")];
    let wrapper = Wrapper(vector);
    println!("wrapper = {}", wrapper);
}
