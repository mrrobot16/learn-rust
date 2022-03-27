fn main() {
    println!("Hello, world of closures and iterators");
    let x = vec![1, 2, 3];

    // let equal_to_x = move |z| z == x;
    let equal_to_x = |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
    
    let v1: Vec<i32> = vec![1, 2, 3];
    let closure = |x| x + 1;
    // v1.iter().map(|x| x + 1);
    let map = v1.iter().map(closure);
    // let map2 = v1.iter().map(closure);
    // println!("closure: {:?}", closure);
    // println!("map: {:#?}", map);
    let v2: Vec<_> = map.collect();
    println!("v2: {:#?}", v2);
    assert_eq!(v2, vec![2, 3, 4]);
}