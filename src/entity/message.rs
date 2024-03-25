
#[derive(Debug)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    pub fn call(&self) {
        match self {
            Message::Quit => println!("quit"),
            Message::Move { x, y } => println!("move x:{x}, y:{y}"),
            Message::Write(text) => println!("write {text}"),
            Message::ChangeColor(r, g, b) => println!("change color r:{r}, g:{g}, b:{b}"),
        }
    }
}
