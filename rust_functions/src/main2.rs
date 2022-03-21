fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1;
    
    // println!("{}, world!", s1);
    // let x = "5";
    // let x = String::from("5");
    // let y = x;
    let s1 = String::from("hello");
    // let s2 = s1;
    let s2 = s1.clone();

println!("{}, world!", s1);
println!("s1 = {}, s2 = {}", s1, s2);

    // println!("x = {}", x);
    // println!("y = {} ", y); 
    // let mut z: &str = "asd1";
    // println!("z1 = {}", z);
    // z = z.to_owned() + "asd2";
    // z = &(z.to_owned() + "asd2");
    // 
    // println!("x = {} y:",y); 
    // println!("z2 = {} z:",z);    
}