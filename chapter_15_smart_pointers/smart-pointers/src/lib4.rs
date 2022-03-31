use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    pub fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        println!("I am calling tail?");
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}

// Old List
// pub enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }