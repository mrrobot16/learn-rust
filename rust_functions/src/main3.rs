fn main() {
    let s = String::from("hello");  // s comes into scope
    let s1 = String::from("world");
    takes_ownership(s);             // s's value moves into the function...
    // println!("s: {}", s);                                // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
    println!("x: {}", x);                                // but i32 is Copy, so it's okay to still
                                    // use x afterward
                                    let (s2, len) = calculate_length(s1);

                                println!("The length of '{}' is {}.", s2, len);        
                                
                                let mut s2 = String::from("hello");

change(&mut s2);      
println!("s2: {}", s2);           
// 
// {
// let r1 = &mut s2;
// println!("r1: {}", r1);
// }
// let r2 = &mut s2;       
// // println!("r1: {}", r1);
// println!("r2: {}", r2);

// let mut s = String::from("hello");

   let r1 = &s2; // no problem
   let r2 = &s2; // no problem
   let r3 = &mut s2; // BIG PROBLEM

   println!("{}, {}, and {}", r1, r2, r3);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens. 

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}