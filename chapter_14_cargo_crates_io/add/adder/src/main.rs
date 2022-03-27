use add_one::{add_one};
// use rand;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
    // println!("Hello, world!");
    println!("Calling add_one fn(add_one(1)) ->  {}", add_one(1));
}
