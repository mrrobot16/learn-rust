fn main() {
    let user1 = User {
        username: String::from("someoneusername123"),
        email: String::from("someone@example.com"),
        // username: "someoneusername123",
        // email: "someone@example.com",
        active: true,
        sign_in_count: 0,
    };
    
    println!("user1: {:?}", user1);
    
    let user2 = User {
        email: String::from("anotheremail@example.com"),
        // email: "anotheremail@example.com"
        ..user1
    };
    
    println!("user2: {:?}", user2);
    
    let my_user_name = String::from("my_user_name");
    let my_email = String::from("my_email@email.com");

    // let my_user_name = "my_user_name";
    // let my_email = "my_email@email.com";
    build_user(my_user_name, my_email);
    
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {:?}", black);
    println!("origin: {:?}", origin);
    
    let subject = AlwaysEqual;
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    // username: &str,
    // email: &str,
    active: bool,
    sign_in_count: u64
}

fn build_user(username: String, email: String) -> User {
// fn build_user(username: &str, email: &str ) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 0,
    }
}

// Tuple Structs
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

// Unit Like Structs without fields
struct AlwaysEqual;
