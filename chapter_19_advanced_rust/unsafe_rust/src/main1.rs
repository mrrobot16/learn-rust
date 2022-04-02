use advanced_rust::{dangerous, split_at_mut};
use std::slice;

fn main() {
    println!("Hello, world of Advanced Rust");
    
    // Raw pointer Unsafe Rust
    
    // 19.1
    let mut number = 5;
    let r1 = &number as *const i32;
    let r2 = &mut number as *mut i32;
    
    println!("r1: {:?}", r1);
    println!("r2: {:?}", r2);
    
    // 19.2 
    let address = 0x012345usize;
    let r = address as *const i32;
    
    // 19.3 Code below will not work, unless we put in an unsafe block below
    // println!("*r1: {}", *r1);
    // println!("*r2: {}", *r2);
    
    unsafe {
        dangerous();
        println!("*r1: {:?}", *r1);
        println!("*r2: {:?}", *r2);
    }
    
    // 19.4 
    let mut vector = vec![1,2,3,4,5,6];
    let r = &mut vector[..];
    let (a, b) = r.split_at_mut(3);
    
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    
    // 19.7 
    let address = 0x01234usize;
    let r = address as *mut i32;
    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 1000) };
}
