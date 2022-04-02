use advanced_traits::{Human, Pilot, Wizard, Dog, Animal};
// use advanced_traits::{Human};

fn main() {
    println!("Hello, world of Advanced Trait!");
    let human = Human;
    Pilot::fly(&human);
    Wizard::fly(&human);
    human.fly();
    
    let dog = Dog;
    // let dog_name = dog.baby_name();
    let dog_name = Dog::baby_name(&dog);
    // let dog_name = Dog::baby_name();
    
    println!("A baby dog is called a {}", dog_name);
    
    // 19.21
    // below code throws Animal::baby_name()); ^^^^^ cannot infer type
    // let animal_name = Animal::baby_name();
    let animal_name = <Dog as Animal>::baby_name();
    println!("A baby dog is called a {}", animal_name);

}
