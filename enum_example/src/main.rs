#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn main() {
    let mut message = Message::Move::Quit;
    println!("{:?}", message);
    message = Message::Move { x: 30, y: 40 };
}