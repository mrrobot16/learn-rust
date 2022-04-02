use unsafe_rust::{add_to_count};

fn main() {
    // Raw pointer Unsafe Rust
    println!("Hello, world of Advanced Rust & Unsafe Rust");
    
    println!("name is {}", HELLO_WORLD);
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    
    add_to_count(3);
}

// 19.9
static HELLO_WORLD: &str = "Hello, world!";

// 19.8 
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

extern "C" {
    fn abs(input: i32) -> i32;
}
