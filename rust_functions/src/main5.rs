fn main() {
    let full_word = String::from("hello world");
    println!("fullword: {}", full_word);
    let fw = first_word(&full_word);
    let sw = second_word(&full_word);

    // println!("fw('hello world'): {}", fw);
    // println!("sw('hello world'): {}", sw);
    
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("word 1: {}", word);
    let word = first_word(&my_string[..]);
    println!("&my_string[..] {}", &my_string[..]);
    println!("word 2: {}", word);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    println!("word 3: {}", word);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("word 4: {}", word);
    let word = first_word(&my_string_literal[..]);
    println!("word 5: {}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("word 6: {}", word);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    // println!("first_word bytes: {:?}", bytes);
    for (i, &item) in bytes.iter().enumerate() {
        // println!("&item {:?}", item);
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]

}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    // println!("second_word bytes: {:?}", bytes);
    for (i, &item) in bytes.iter().enumerate() {
        // println!("&item {:?}", item);
        if item == b' ' {
            return &s[i+1.. s.len()];
        }
    }
    
    &s[..]
}