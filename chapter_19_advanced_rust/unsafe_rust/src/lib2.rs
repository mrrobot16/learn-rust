// 19.10
static mut COUNTER: u32 = 0;

pub fn add_to_count(inc: u32) {
    unsafe {
        println!("Counter: {}", COUNTER);
        COUNTER += inc;
        println!("Counter: {}", COUNTER);
    }
}
