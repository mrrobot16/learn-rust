fn main() {
    println!("Hello, world of Pattern and Matching!");
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    
    // 18.11
    let x = Some(5);
    let y = 10;
    
    match x {
        Some(50) => println!("Got 50"),
        Some(z) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    
    println!("at the end: x = {:?}, y = {:?}", x, y);
    
    let x = 3; 
    
    match x {
        1 | 2 | 4 => println!("one, two or four"),
        3 => println!("three"),
        6..= 10 => println!("6 through 10"),
        _ => println!("anything")
    }
    
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}
