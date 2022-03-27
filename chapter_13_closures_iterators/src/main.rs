use std::thread; 
use std::time::Duration;

fn main() {
    println!("Hello, world of closures and iterators");
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(a: u32, b: u32) -> u32 {
    return a + b
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly");
    thread::sleep(Duration::from_secs(2));
    intensity
}