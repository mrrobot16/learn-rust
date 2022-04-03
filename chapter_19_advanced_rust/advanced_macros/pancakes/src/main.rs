use advanced_macros::HelloMacro;
// use hello_macro_derive::HelloMacro;

// #[derive(HelloMacro)]
struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is PancakeSwap!");
    }
}

fn main() {
    println!("Hello, world of Advanced Macro Pancakes!!!!");
    Pancakes::hello_macro();
    // let sql = sql!("SELECT * FROM posts WHERE id=1");
}
