use gui::{Button, Screen, SelectBox};

fn main() {
    println!("Hello world of OOP GUI");
    
    let select_box = Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
            String::from("Yes"),
            String::from("Maybe"),
            String::from("No"),
        ],
    });
    
    let button = Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("OK"),        
    });
    
    let screen = Screen {
        components: vec![
            select_box,
            button,
        ]
    };
    
    screen.run();
}
