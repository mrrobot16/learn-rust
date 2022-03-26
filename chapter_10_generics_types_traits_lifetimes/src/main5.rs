fn main() {
    println!("Hello, world of generics!");
    
//     let string1 = String::from("abcd");
// let string2 = "xyz";
// 
// let result = longest(string1.as_str(), string2);
// println!("The longest string is {}", result);

// let string1 = String::from("long string is long");
// 
// {
//     let string2 = String::from("xyz");
//     let result = longest(string1.as_str(), string2.as_str());
//     println!("The longest string is {}", result);
// }

let string1 = String::from("long string is long");
let result;
{
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
}
println!("The longest string is {}", result);

    // let r;

// {
//     let x = 5;
//     let r = &x;
//     println!("r: {}", r);
// }


}

fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }