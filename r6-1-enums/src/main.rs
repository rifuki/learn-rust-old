enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u32, u32, u32)
}

fn main() {
    let msg1 = Message::Write(String::from("new message"));
}
