use std::thread;
// use std::time::Duration;

fn main() {
    println!("Hello, world of Concurrency");
    let vector = vec![1, 2, 3];
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
    
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    // 
    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // In order to make this callback access "vector" variable 
    // we need to add the "move" keyword before the closure.
    // let callback = || {
    // let callback = move || {
    //     println!("Here's a vector: {:?}", vector);
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // };
    // 
    // let handle = thread::spawn(callback);
    // handle.join().unwrap();
    // 
    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    // handle.join().unwrap();
}
