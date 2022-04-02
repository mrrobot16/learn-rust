use std::fmt;
use std::io::Error;

// 19.24
// avoid // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
// instead type CustomType = Box<dyn Fn() + Send + 'static>;
type CustomType = Box<dyn Fn() + Send + 'static>;

pub type Thunk = CustomType;

fn takes_long_type(f: CustomType) {}
// fn returns_long_type() -> CustomType { }

// Example of type used, it wont work thougt need to fix arguement error.
// pub trait Write<> {
//     type Result<T> = std::result::Result<T, std::io::Error>;
//     fn write(&mut self, buf: &[u8]) -> Result<usize>;
//     fn flush(&mut self) -> Result<()>;
// 
//     fn write_all(&mut self, buf: &[u8]) -> Result<()>;
//     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
// }

// 19.26 Never Type that never returs
// pub fn bar() -> ! {
// 
// }

// When using T
fn generic<T>(t: T) {
    // --snip--
}
// is actually treated as though we had written this:
fn generic2<T: Sized>(t: T) {
    // --snip--
}

// By default, generic functions will work only on types that have a known size at compile time. 
// However, you can use the following special syntax to relax this restriction:
fn generic3<T: ?Sized>(t: &T) {
    // --snip--
}
