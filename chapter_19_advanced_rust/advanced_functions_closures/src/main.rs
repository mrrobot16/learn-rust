
fn main() {
    println!("Hello, world of Advance Functions ands closures!");
    // 19.27    
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
    
    let list_of_numbers = vec![1, 2, 3];
    // let list_of_numbers_to_string = |i: i32 | i.to_string();
    // let list_of_numbers_to_string = ToString::to_string;
    let list_of_strings: Vec<String> =
        // list_of_numbers.iter().map(|i| i.to_string()).collect();
        list_of_numbers.iter().map(ToString::to_string).collect();
    let list_of_statuses: Vec<Status> = (0u32..10).map(Status::Value).collect();
    println!("list_of_statuses: {:?}", list_of_statuses);
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    let closure = |x| x + 1;
    Box::new(closure)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

// 19.27
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

