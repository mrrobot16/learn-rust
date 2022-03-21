fn main() {
    println!("Enums are great!");
    
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    let message = Message::Write(String::from("hello"));
    // message.call();
    
    // let cents = values_in_cents(Coin::Penny);
    
    // let quarter_cents = values_in_cents(Coin::Quarter(UsState::Florida));
    
    // println!("values_in_cents(Coin::Penny): {}", cents);
    
    let mut count = 0;
    
    let quarter_fl = Coin::Quarter(UsState::Florida);
    
    let quarter_ny = Coin::Quarter(UsState::NewYork);
    
    let quarter_ca = Coin::Quarter(UsState::California);
    
    // let quarter_al = Coin::Quarter(UsState::Alabama);
    
    println!("count 0: {}", count);
    match quarter_ny {
        Coin::Quarter(ref state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    if let Coin::Quarter(state) = quarter_ny {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    
    println!("count 1: {}", count);
    match quarter_ca {
        Coin::Quarter(ref state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    if let Coin::Quarter(state) = quarter_ca {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1
    }
    
    // println!("count 2: {}", count);
    // match quarter_al {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }
    // if let Coin::Quarter(state) = quarter_al {
    //     println!("State quarter from {:?}!", state)
    // } else {
    //     count += 1
    // }
    
    println!("count 3: {}", count);
    match quarter_fl {
        Coin::Quarter(ref state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    if let Coin::Quarter(state) = quarter_fl {
        println!("State quarter from {:?}!", state)
    } else {
        count += 1
    }

    // Soon we fix this in advanced chapters
    let some_number = Some(5);
    let some_string = Some("a string");
    
    let absent_number: Option<i32> = None;
    
    // println!("absent_number: {:?}", absent_number);
    
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    
    let one = plus_one(Some(0));
    // println!("one: {:?}", one);
    
    let six = plus_one(Some(5));
    // println!("five: {:?}", six);
    
    let none = plus_one(None);
    // println!("none: {:?}", none);
    // let sum = x + y;
    // roll_dices();
    
    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }
    // 
    // if let Some(max) = config_max {
    //     // println!("The maximum is configured to be {}", max)
    // }

}

fn roll_dices() {
    let rolls = [7, 3, 6, 5, 9, 8];
    
    let dice_roll = 7;
    match dice_roll {
      3 => add_fancy_hat(),
      7 => remove_fancy_hat(),
      other => move_player(other),
    }
      
    let dice_roll = 3;
    match dice_roll {
      3 => add_fancy_hat(),
      7 => remove_fancy_hat(),
      other => move_player(other),
    }
      
    let dice_roll = 6;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        6 => remove_fancy_hat(), // THIS WILL NOT BE CHECK BECAUSE IS AFTER THE CATCH-ALL PATTERN "other".
    }
        
    let dice_roll = 5;
      match dice_roll {
          3 => add_fancy_hat(),
          7 => remove_fancy_hat(),
          // other => move_player(other),
          _ => move_player(2),
      }
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
        _ => move_player(2)
    }    
    let dice_roll = 8;
    match dice_roll {
      3 => add_fancy_hat(),
      7 => remove_fancy_hat(),
      _ => (),
    }
}

fn add_fancy_hat() {
    println!("3 was rolled");
}
fn remove_fancy_hat() {
    println!("7 was rolled");
}
fn move_player(num_spaces: u8) {
    println!("other called num_spaces: u8: {}", num_spaces);
}

#[derive(Debug)]
enum UsState {
    Florida,
    NewYork,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn values_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}



enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Do this
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message.call()");
        // method body would be defined here
    }
}

// AVOID THIS.
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
