use smart_pointers::List::{Cons, Nil};
// use smart_pointers::{hello, MyBox};
// use smart_pointers::{CustomSmartPointer};
use std::rc::Rc;
use std::cell::RefCell;


fn main() {
    println!("Hello, world of Smart Pointers");
    
    let value = Rc::new(RefCell::new(5));
    println!("value = {:#?}", value);
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    
    *value.borrow_mut() += 10;
    
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    
    println!("*value.borrow_mut() += 10");
    
    println!("value = {:?}", value);
    
    // Below code throws error.
    // let x = 5;
    // let y = &mut x;
    
    // let ref_counter = Cons(5, Rc::new(Cons(10, Rc::new(Nil))));
    // let ref_a = Rc::new(ref_counter);
    // println!("count after creating ref_a = {}", Rc::strong_count(&ref_a));
    // let _ref_b = Cons(3, Rc::clone(&ref_a));
    // println!("count after creating ref_b = {}", Rc::strong_count(&ref_a));
    
    // let ref_c = Cons(4, Rc::clone(&ref_a));
    // {
    //     let _ref_c = Cons(4, Rc::clone(&ref_a));
    //     println!("count after creating ref_c = {}", Rc::strong_count(&ref_a));
    // }
    // 
    // println!("count after creating ref_c = {} end scope, should equal to ref_b's count", Rc::strong_count(&ref_a));
    
    // println!("ref_a: {}", ref_a);
    // println!("ref_b: {}", _ref_b);
    // println!("ref_c: {}", _ref_c);
    
    // Uncomment in List type for this to fix type error, this is not gonna work anyways.
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));    
    // println!("a: {}", a);
    // println!("b: {}", b);
    // println!("c: {}", c);

    // let csp1 = CustomSmartPointer {
    //     data: String::from("csp1 data")
    // };
    // 
    // println!("CustomSmartPointer created! {:?}", csp1);
    // drop(csp1);
    
    // println!("CustomSmartPointer created! {:?} 1", csp1);
    // drop(csp1);
    // println!("CustomSmartPointer creation ended!");
    
    // let csp2 = CustomSmartPointer {
    //     data: String::from("csp2 data")
    // };
    // 
    // let csp3 = CustomSmartPointer {
    //     data: String::from("csp3 data")
    // };
    
    // println!("CustomSmartPointer created!");

    // NOTE "*" dereference operator to follow a reference to an i32 value. 
    //  "*" dereference operator only works on &reference.
    // let x = 5;
    // let y = MyBox::new(x);
    // println!("y: {}", *y);
    // assert_eq!(5, *y);
    // let some_string = String::from("SomeString");
    // let z = MyBox::new(some_string);
    // println!("z: {:?}", *z);
    // hello(&z);
    // 
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
