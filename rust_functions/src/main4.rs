fn main() {
//     let mut s = String::from("hello");
// 
// let r1 = &s; // no problem
// let r2 = &s; // no problem
// let r3 = r2;
// println!("{} and {}", r1, r2);
// variables r1 and r2 will not be used after this point
// println!("r1{} and r2{}", r1, r2);

// let r3 = &mut s; // no problem
// let r3 = r2;
// println!("r1{} and r2{}", r1, r2);
// println!("{}", r3);
let full_word = String::from("hello world");
println!("fullword: {}", full_word);
let fw = first_word(&full_word);
let sw = second_word(&full_word);
// let fw = first_word(&String::from("hello world"));

println!("fw('hello world'): {}", fw);
println!("sw('hello world'): {}", sw);


// let s = String::from("hello");
// 
// let slice = &s[0..2];
// println!("slice 1: {}", slice);
// let slice = &s[..2];
// println!("slice 2: {}", slice);

// let s = String::from("hello");
// 
// let len = s.len();
// 
// let slice = &s[0..len];
// println!("slice 1: {}", slice);
// let slice = &s[..];
// println!("slice 2: {}", slice);
// let slice = &s[..len];
// println!("slice 3: {}", slice);
}

// fn dangle() -> String {
//     let s1 = String::from("Dangle()");
//     s1
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    // println!("first_word bytes: {:?}", bytes);
    for (i, &item) in bytes.iter().enumerate() {
        // println!("&item {:?}", item);
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]

    // s.len()
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    // println!("second_word bytes: {:?}", bytes);
    for (i, &item) in bytes.iter().enumerate() {
        // println!("&item {:?}", item);
        if item == b' ' {
            return &s[i+1.. s.len()];
        }
    }
    
    &s[..]
}