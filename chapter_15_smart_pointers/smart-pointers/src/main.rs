// use smart_pointers::List::{Cons, Nil};
use smart_pointers::{hello, MyBox};

fn main() {
    println!("Hello, world of Smart Pointers");
    // NOTE "*" dereference operator to follow a reference to an i32 value. 
    //  "*" dereference operator only works on &reference.
    // let x = 5;
    // let y = MyBox::new(x);
    // println!("y: {}", *y);
    // assert_eq!(5, *y);
    // let some_string = String::from("SomeString");
    // let some_string = "SomeString";
    // let z = MyBox::new(some_string);
    // println!("z: {:?}", *z);
    // hello(&z);
    
    // let x = 5;
    // let y = &x;
    // let y = Box::new(x);
    // println!("y: {}", y);
    // println!("*y: {}", *y);


    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    
    // let boxed = Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))));
    // println!("boxed = {}", boxed);
    // let list = Cons(1, boxed);
    
    // println!("list = {:?}", list);
}
