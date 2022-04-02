use match_pattern::{Point, Color, Message, MessageSender};

fn main() {
    println!("Hello, world of Pattern and Matching!");
    
    // 18.13
    let point = Point { x: 0, y: 7, z: 10 };
    let Point { x: a, y: b, .. } = point;
    let Point { x, y, .. } = point;
    assert_eq!(0, a);
    assert_eq!(7, b);
    assert_eq!(0, x);
    assert_eq!(7, y);
    
    // 18.14 
    match point {
        Point { x, y: 0, .. } => println!("On the x axis at {}", x),
        Point { x: 0, y, .. } => println!("On the y axis at {}", y),
        Point { x, y, .. } => println!("On neither axis: ({}, {})", x, y), 
    }
    
    // 18.15 + 18.16
    let message = Message::ChangeColor(Color::Rgb(0, 169, 255));
    // let message = Message::Write("write 0, 160, 255".to_string());
    // let message = Message::Quit;
    match message {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        },
        Message::Write(text) => println!("Text message: {}", text),
        
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
    
    // 18.17
    let ((feet, inches), Point { x, y, .. }) = ( (3, 10), Point { x: 3, y: -10, z: 0 } );
    
    // 18.18 
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    println!("setting first is: {:?}", setting_value);
    match (setting_value, new_setting_value) { 
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        },
        _ => { 
            setting_value = new_setting_value;
        }
    }
    
    println!("setting now is: {:?}", setting_value);
    
    // 18.19
    
    let numbers = (2, 4, 8, 16, 32);
    
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
    
    // 18.21 
    let s = Some(String::from("Hello!"));

     // if let Some(_s) = s {
     //     println!("found a string");
     // }
     if let Some(_) = s {
         println!("found a string");
     }

     println!("{:?}", s);
     
     // 18.23 
     
     let origin = Point { 
         x: 0,
         y: 0,
         z: 0,
     };
     
     match origin {
         Point { x, .. } => println!("x is: {}", x),
     }
     
     // 18.24 
     let numbers = (2, 4, 8, 16, 32);
     match numbers {
         (first, .., last) => {
             println!("Some numbers: 2:{}, 32:{}", first, last);
         }
     }
     
     // 18.25
     // this is will throw error
     // match numbers {
     //     (.., second, ..) => {
     //         println!("some numbers: second: {}", second);
     //     }
     // }
     
     // 18.26
     
     // let number = Some(4);
     let number = Some(10);
     match number {
         Some(x) if x < 5 => println!("less than five: {}", x),
         Some(x) => println!("x: {}", x),
         None => ()
     }
     
     // 18.27
     let x = Some(5);
     let y = 10;
     
     match x {
         Some(50) => println!("Got 50"),
         Some(n) if n == y => println!("Matched, n = {}", n),
         _ => println!("Default case, x = {:?}", x),
     }
     
     println!("at the end: x = {:?}, y = {}", x, y);
     
     // 18.28
     let x = 4;
     let y = false;
     
     match x {
         4 | 5 | 6 if y => println!("yes"),
         _ => println!("no"),
     }
     
     // 18.29
     let message = MessageSender::Hello { id: 5 };
     // let message = MessageSender::Bye { id: 15 };
     match message {
         MessageSender::Hello {
             id: id_variable @ 3..=7,
         } => println!("Found an id in range: {}", id_variable),
         
         MessageSender::Hello { id: id_variable @10..=12 } => {
             println!("Found an id in another range: {}", id_variable)
         },
         MessageSender::Hello { id } => println!("Found some other id: {}", id),
         
         MessageSender::Bye { id } => println!("Found an id for Bye enum: {}", id),
     }
     
}
