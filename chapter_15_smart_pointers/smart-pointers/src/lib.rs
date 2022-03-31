use std::fmt;
use std::ops::{Deref, Drop};
use std::rc::{Rc,};

#[derive(Debug)]
pub struct CustomSmartPointer {
    pub data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[derive(Debug)]
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub enum List {
    // Cons(i32, Box<List>),
    // NOTE Below code does not work because Cons second value need to have a size known like 
    // ie: A Box smart pointer.
    // Cons(i32, List),
    
    // reference cointing list.
    Cons(i32, Rc<List>),
    Nil,
}


impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self)
    }
}

pub fn hello(name: &str) {
    println!("Hello, {}!", name);
}