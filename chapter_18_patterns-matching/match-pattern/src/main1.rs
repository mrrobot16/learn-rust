fn main() {
    println!("Hello, world of Pattern and Matching!");
    // 18.1
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
    
    // 18.2
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    
    // 18.3
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    
    // 18.4
    let (x, y, z) = (1, 2, 3);
    
    // 18.5 this code below throws mismatched types & "expected a tuple with 3 elements, found one with 2 elements"
    // remove one number from tuple. 
    // let (x,y) = (1,2,3);
    
    if let Some(x) = Some("some_option_value") {
        println!("{}", x);
    }
    
    if let x = 5 {
        println!("warning: {}", x);
    }
}
