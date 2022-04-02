use advanced_types::{Thunk};

fn main() {
    println!("Hello, world of Advance Types");
    
// 19.25
    let thunk: Thunk = Box::new(|| println!("hi"));
    let number_list = vec![34, 50, 25, 100, 65];
// 19.26
    let guess = "30";
    loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            // "continue" part only works if it is inside loop.
            Err(_) => continue
        };
        break;
    }
}

// 19.25
fn takes_long_type(f: Thunk) {
    
}

// fn returns_long_type() -> Thunk {
// 
// }




