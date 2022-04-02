// 19.16
pub struct Human;

// This struct is not needed if the structs have no parameters and a trait is defined.
// struct Pilot;
// struct Wizard;

pub trait Pilot {
    fn fly(&self);
}

pub trait Wizard {
    fn fly(&self);
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    pub fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// 19.19
pub struct Dog;

pub trait Animal {
    fn baby_name() -> String;
}

impl Dog {
    pub fn baby_name(&self) -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}