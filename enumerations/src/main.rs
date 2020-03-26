#[derive(Debug)]
enum Message {
    Write(String),
    Move{x : u32, y : u32},
    ChangeColor{r : u8, g : u8, b : u8},
}

impl Message {
    fn call(&self) {
        match self {
            Self::Write(message) => {
                println!("Write message: {}", message);
            },
            Self::Move{x, y} => {
                println!("Move message: {}, {}", x, y);
            },
            _ => {
                println!("Other message");
            }
        }
    }
}

fn main() {
    Message::Write(String::from("Hello Enumeration!")).call();
    Message::Move{ x : 100, y : 200}.call();
    Message::ChangeColor{ r : 100, g : 200, b : 100}.call();
}
