use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world of Concurrency");
    
    // open a channel.
    let (tx, rx) = mpsc::channel();
        
    // send multiple messages from multiple threads to one receiver.
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    // This Thread will execute first.
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    // send multiple messages using a vector. using one thread.
    thread::spawn(move || {
        let vals = vec![
            String::from("hi2"),
            String::from("from2"),
            String::from("the2"),
            String::from("thread2"),
        ];
        for val in vals {
            // tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
    
    // thread::spawn(move || {
        // let val = String::from("hi receiver");
        // tx.send(val).unwrap();
        // val is no longer in this scope because val's ownership is transfered to received.
        // println!("val is {}", val);
    // });
    
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
}
