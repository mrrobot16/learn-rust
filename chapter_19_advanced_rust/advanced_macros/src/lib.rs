// "Declarative Macros"", "Macros by example", "macro_rules!", "macros"

// Declarative Macros
// 19.28
pub fn define_variables() {

    let vector: Vec<u32> = vec![1, 2, 3];
}
// vec! macro defined
#[macro_export]
macro_rules! vec {
    ( $( $x: expr), * ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// 19.29
// Procedual macros
// Types: All work in similar fashion
// - Custom derive 
// - attribute-like
// - function-like

// Example Procedual macro
// use proc_macro;
// 
// #[some_attribute]
// pub fn some_name(input TokenStream) -> TokenSteam {}

pub trait HelloMacro {
    fn hello_macro();
}


// #[derive(HelloMacro)]
pub struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}
