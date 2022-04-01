use std::sync::Mutex;

fn main() {
    println!("Hello, world of Concurrency");
    let mutex = Mutex::new(50);
    {
        let mut number = mutex.lock().unwrap();
        *number = 6;
    }
    println!("mutex = {:?}", mutex);
}