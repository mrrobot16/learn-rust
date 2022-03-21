// use self::front_of_house::hosting;
pub use crate::front_of_house::hosting;
// use self::front_of_house::hosting::add_to_waitlist;
// use crate::front_of_house::hosting::add_to_waitlist;

// use std::fmt;
// use std::io;

use std::fmt::Result;
use std::io::Result as IoResult;

// fn function1() -> fmt::Result {
//     // --snip--
// }
// 
// fn function2() -> io::Result<()> {
//     // --snip--
// }

// fn function1() -> fmt::Result {
    // --snip--
// }

// fn function2() -> IoResult<()> {
    // --snip--
// }

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
