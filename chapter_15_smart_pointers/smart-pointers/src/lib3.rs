use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> 
where 
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    } 
    
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        
        let percentage_of_max = self.value as f64 / self.max as f64;
        
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
        else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        }
        else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[derive(Debug)]
pub enum List {
    // Cons(i32, Box<List>),
    // NOTE Below code does not work because Cons second value need to have a size known like 
    // ie: A Box smart pointer.
    // Cons(i32, List),
    
    // reference cointing list.
    // Cons(i32, Rc<List>),
    
    // Combining RefCell with Rc.
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}


impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self)
    }
}


#[cfg(test)]
mod tests {
    use super::{Messenger, LimitTracker, Rc, RefCell};

    
    // Mock data
    struct MockMessenger {
            sent_messages: RefCell<Vec<String>>,
        }

        impl MockMessenger {
            fn new() -> MockMessenger {
                MockMessenger {
                    sent_messages: RefCell::new(vec![]),
                }
            }
        }

        impl Messenger for MockMessenger {
            fn send(&self, message: &str) {
                // self.sent_messages.borrow_mut().push(String::from(message));
                
                // this is gonna make test fail even though is gonna compile.
                let mut one_borrow = self.sent_messages.borrow_mut();
                let mut two_borrow = self.sent_messages.borrow_mut();

                one_borrow.push(String::from(message));
                two_borrow.push(String::from(message));
                
                
            }
        }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut mock_limit_tracker = LimitTracker::new(&mock_messenger, 100);
        
        mock_limit_tracker.set_value(80);
        
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}