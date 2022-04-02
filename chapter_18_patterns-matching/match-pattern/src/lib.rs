pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

pub enum MessageSender {
    Hello { id: i32 },
    Bye { id: i32 },
}