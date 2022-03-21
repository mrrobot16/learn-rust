use std::io;

fn main() {
    println!("Hello, world!");
    
    let x = 5;
    // The below code throws error. Uncomment below
    let mut x = x;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);

    let x = 5;
    let x = x + 1;
    
    {
        let x = x * 2;
        println!("Shadowing The value of x in the inner scope is: {}", x);
    }

    println!("Shadowing The value of x is: {}", x);    
    
    // This code throws error works
    let spaces = "   ";
    let spaces = spaces.len();
    
    // this works
    // let spaces = "   ";
    // let spaces = spaces.len();
    
    println!("spaces.len: {}", spaces);
    
    let x: &str = "my name";
    println!("x: {}", x);
    let x: char  = 'X'; 
    println!("x: {}", x);
    
    // Tuple
    let tup: (&str, char, f64, u8, i128) = ("some_string", 'C', 1_000.50, 10, -8);
    
    println!("tup some_string: {}", tup.0);
    
    let (v,w,x,y,z) = tup;
    
    println!("v: {}", v);
    println!("w: {}", w);
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z); 

    // Array  
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
        
    println!("first: {}", first);
    println!("second: {}", second);

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
    
    
}
