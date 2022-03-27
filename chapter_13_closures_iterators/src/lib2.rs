#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    let shoes = shoes.into_iter();
    let closure = |shoe: &Shoe | shoe.size == shoe_size;
    let filtered_shoes = shoes.filter(closure).collect();
    filtered_shoes
    // or in one line use below.
    // shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}


#[cfg(test)]
mod tests {
    use super::{Shoe, shoes_in_size};
    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        
        let in_my_size = shoes_in_size(shoes, 10);
        
        let my_size_shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ];
        assert_eq!(in_my_size, my_size_shoes);
    }
}