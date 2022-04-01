use std::sync::{Mutex, Arc};
use std::thread;
use std::rc::Rc;

fn main() {
    println!("Hello, world of Concurrency");
    
    let counter = Rc::new(Mutex::new(0));
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    // 
    for _ in 0..10 {
        // this below give errror, for ref counting inside multiple threads is better to use Arc.
        // let counter = Rc::clone(&counter);
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
    
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
    
    // let mutex = Mutex::new(50);
    // {
    //     let mut number = mutex.lock().unwrap();
    //     *number = 6;
    // }
    // println!("mutex = {:?}", mutex);
}