use::std::collections::HashMap;
fn main() {
    println!("Hello, world of Common Collections!");
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);    
//     let mut scores = HashMap::new();
//     println!("{:?}", scores);
// scores.insert(String::from("Blue"), 10);
// println!("{:?}", scores);
// scores.entry(String::from("Yellow")).or_insert(50);
// println!("{:?}", scores);
// scores.entry(String::from("Red")).or_insert(50);
// 
// println!("{:?}", scores);

// let field_name = String::from("Favorite color");
// let field_value = String::from("Blue");

// let mut map = HashMap::new();
// println!("field_name {}", field_name);
// println!("field_value {}", field_value);
// map.insert(field_name, field_value);
// println!("map: {:?}", map);
// println!("field_name {}", field_name);
// println!("field_value {}", field_value);
    // let mut scores = HashMap::new();
    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];
    // 
    // let mut scores: HashMap<_, _> =
    //     teams.into_iter().zip(initial_scores.into_iter()).collect();
    //     let team_name = String::from("Blue");
    //     let score = scores.get(&team_name);
    //     // println!("score: {:?}", score);
    //     for (key, value) in &scores {
    // println!("{}", value);
// }
// println!("scores {:?}", scores);
// scores.insert("Asd".to_string(), 100);
// scores.insert(String::from("Blue"), 10);
// println!("scores {:?}", scores);
// scores.insert(0, 50);
// println!("scores {:?}", scores);
// scores.insert(0, 500);
// scores.insert(String::from("Yellow"), 50);
// println!("scores {:?}", scores);
    // let s1 = String::from("hello");
    // let h = s1[0];
//     let hello = "Здравствуйте";
// let answer = &hello[0];

// let hello = "Зpдавствуйте";

// let s = &hello[0..2];
// println!("{}",s);
// let chars = "नमस्ते".chars();
// println!("a: {:?}", a);
// for c in chars {
//     println!("{}", c);
// }
// println!("{}",s);

// for b in "नमस्ते".bytes() {
//     println!("{}", b);
// }

// let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");
// 
//     let s = format!("{}-{}-{}", s1, s2, s3);
// 
// 
//     let s = s1 + "-" + &s2 + "-" + &s3;
//     println!("s is {}", s);
    // println!("****************");
    // let s = format!("{}-{}-{}", s1, s2, s3);
    // println!("s is {}", s);
    // let data = "initial contents1";
    // let mut s = String::from("foo");
    // let mut s1 = String::from("foo");
    // println!("s1 is {}", s1);
   // let s2 = "bar";
   // println!("s1 is {}", s1);
   // s1.push_str(s2);
   // let s1 = String::from("Hello, ");
   // let s2 = String::from("world!");
       // println!("s1 is {}", s1);
   //     let w = "213";
   //     let v = w.to_owned() + w;
   // let s3 = s1 + &s2;
   // let s1 = String::from("tic");
   //     println!("s1 is {}", s1);
   //  let s2 = String::from("tac");
   //  let s3 = String::from("toe");
   //  println!("s1 is {}", s1);
   //  let s = s1 + "-" + &s2 + "-" + &s3;
        // println!("s1 is {}", s1);
    // println!("s2 is {}", s2);
    // println!("s is {}", s);
    // 
    // println!("s3 is {}", s3);
    // s = "asdasd".to_string();
   // s.push_str("bar");

// let s = data.to_string();
// println!("{}", data);
// println!("{}", s);
// println!("{}", data);
// the method also works on a literal directly:
// let s = "initial contents".to_string();
// println!("{}", s);
// println!("{}", data);

    
    // let mut vec = Vec::new();
    // vec.push(1);
    // vec.push(2);
    
    // assert_eq!(vec.len(), 2);
    // assert_eq!(vec[0], 1);

    // assert_eq!(vec.pop(), Some(2));
    // assert_eq!(vec.len(), 1);

    // vec[0] = 7;
    // vec[1] = 0;
    // vec.push(3);
    // vec[2] = 4;
    // println!("{}", vec[1]);
    // assert_eq!(vec[0], 7);
    
    // for x in vec {
    //     println!("&vec {}", x);
    // }
    

    // vec.extend([5, 6, 8, 19].iter().copied());
    // println!("vec:{:?}", vec);
    // println!("vec[3]:{:?}", vec[3])
    // for x in vec {
        // println!("&vec {}", x);
    // }
    // assert_eq!(vec, [7, 1, 2, 3]);
// let a = vec.pop();
// println!("a {:?}", a);

// assert_eq!(vec.len(), 2);
// assert_eq!(vec[0], 1);

// assert_eq!(vec.pop(), Some(2));
    // assert_eq!(vec[1], 3);
// assert_eq!(vec.len(), 1);

    // let my_vector = vec![1,2,3,4,5,6,7];
    // let mut my_vector = Vec::new();
    // This fails

    // my_vector.push(6);
    // let first = &my_vector[0];
    // println!("The first element is: {}", first);
    // 
    // 
    // my_vector.push(1);
    // my_vector.push(2);
    // my_vector.push(3);
    // my_vector.push(4);
    // my_vector.push(5);
    // my_vector.push(6);
    // &my_vector.push(7);
    // println!("my_vector: {:?}", my_vector);
    
    // for index in &my_vector {
    //     *index += 50;
    // }


    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{}", i);
    // }
    // 
    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    // }
    // let third: &i32 = &my_vector[2];
    // println!("my_vector's third element is: {}", third);
    // 
    // match my_vector.get(2) {
    //     // third here is suppose to be whatever value my_vector.get(2) is.
    //     Some(third) => println!("my vector's third element inside a match is: {}", third),
    //     None => println!("Number 3 as element is not found as in our third element of my_vector"), 
    // }
    // 
    // let does_not_exist = &my_vector[100];
    // println!("does_not_exist 1: {:?}", does_not_exist);
    // let does_not_exist = my_vector.get(100);
    // println!("does_not_exist 2: {:?}", does_not_exist);
}