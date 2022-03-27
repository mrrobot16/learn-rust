use std::thread; 
use std::time::Duration;

fn main() {
    println!("Hello, world of closures and iterators");
    let simulated_user_specified_value = 26;
    let simulated_random_number = 4;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}



struct Cacher<T>
where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T> 
where 
    T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(value) => value,
            None => {
                let value = (self.calculation)(arg);
                self.value = Some(value);
                value
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|number| {
        println!("calculating slowly from an expensive closure");
        thread::sleep(Duration::from_secs(2));
        number
    });

    // let expensive_result = expensive_closure(intensity);
    
    if intensity < 25 {
        println!("Today do {} pushups", expensive_closure.value(intensity));
        println!("Next, do {} situps", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes", expensive_closure.value(intensity));
        }
    }
}