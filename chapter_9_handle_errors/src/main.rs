use std::fs::File;
// use std::io::ErrorKind;
use std::io::{self, Read};
use std::fs;

fn main() {
    println!("Hello, world of Handling Errors!");
    let f = File::open("hello3.txt").unwrap();
    // let f = read_username_from_file();
    // println!("f: {:?}", f);
    // let username = read_username_from_file();
    // println!("username: {:?}", username);
    // let mut s = String::new();
    // let file = File::open("test_file.txt")?.read_to_string(&mut s)?;
    // let file = File::open("test_file.rs").unwrap();
    // let file = File::open("main.rs").expect("main.rs is not found in root.");
    // println!("{:?}", file);
    
    // let file  = match file {
    //     Ok(file) => {
    //         println!("File found");
    //         println!("file {:?}", file);
    //         file
    //     },
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("some_file.txt") {
    //             Ok(file_created) => file_created,
    //             Err(error) => panic!("Problem creating file :/ {:?}", error),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    // panic!("I NEED HELP");
    // println!("IT IS ALL GOOD NOW");
//     let v = vec![1, 2, 3];
// 
// v[99];
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
// 
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
// 
//     let mut s = String::new();
// 
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }


// fn read_username_from_file() -> Result<String, io::Error> {
//     println!("read_username_from_file()");
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let hello = fs::read_to_string("hello.txt");
//     println!("hello: {:?}", hello);
//     hello
// }

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    println!("s1: {:?}", s);
    File::open("hello.txt")?.read_to_string(&mut s)?;
    println!("s2: {:?}", s);
    Ok(s)
}