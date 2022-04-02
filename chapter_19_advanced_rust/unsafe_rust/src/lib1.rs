use std::slice;

pub unsafe fn dangerous() {
    println!("dangerous sh1t");
}

// 19.5
// this does not work becuase it is unsafe
// pub fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
// 
//     assert!(mid <= len);
// 
//     (&mut slice[..mid], &mut slice[mid..])
// }

// 19.6
pub fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    
    assert!(mid <= len);
    
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}