fn main() {
    println!("Hello, world of generics!");
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    
    let char_list = vec!["y", "m", "a", "q"];
    let result = largest(&char_list);
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
