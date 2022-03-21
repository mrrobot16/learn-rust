use std::io;

fn main() {
    println!("Hello, world!");
    test_function();
    let five_plus_one = plus_one(five());
    let five_plus_one_plus_five = five() + plus_one(five());
    println!("five_plus_one: {}", five_plus_one);
    println!("five_plus_one_plus_five: {}", five_plus_one_plus_five);
    
    control_flow();
    control_flow_let();
    control_flow_loop();
    control_flow_return_values();
    control_flow_while_loop();
    control_flow_loop_collections();
    control_flow_loop_collections_for();
    control_flow_loop_for_range_rev();
}

fn test_function() {
    let y = {
        let x = 3;
        x + 3
    };

    println!("The value of y is: {}", y);

    let number_five = five();
    println!("the value of five is: {:?}", number_five);
}

fn five() -> u8 {
    return 5;
}

fn plus_one(number: u8) -> u8 {
    return number + 1;
}

fn control_flow() {
    println!("Input a number");
    let mut number = String::new();       
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");    
    let number: u32 = number.trim().parse().expect("Please type a number");

    if number > 50 {
        println!("number is > 50");
    } 
    else if number == 50 {
        println!("number is 50")
    }
    else if number > 40 {
        println!("number is > 40");
    }
    else {
        println!("number is < 50 and not 50")
    }
}

fn control_flow_let() {
    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("the value of number is: {}", number);

    // Below throws error because number variable can only be integer and not possibly a string
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is: {}", number);
}

fn control_flow_loop() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 40 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count)
}

fn control_flow_return_values() {
    let mut counter: u128 = 0;

    let result = loop {
        counter += 1;

        if counter == 1000_000_000_0 {
            // break counter * 2;
            break counter
        }
    };

    println!("the result is {}", result);
}

fn control_flow_while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("TAKE OFF");
}

fn control_flow_loop_collections() {
    let a = [1, 2,3,3,23,24,5,92,2,6,23,32567,0];
    let mut index = 0;
    while index < 10 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

fn control_flow_loop_collections_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is {}", element);
    }
}

fn control_flow_loop_for_range_rev() {
    for number in (1..100).rev() {
        println!("reverse number: {}!", number);
    }
    println!("LIFT OFF!!");
}
