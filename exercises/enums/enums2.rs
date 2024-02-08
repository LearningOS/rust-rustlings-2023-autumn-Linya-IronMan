// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move { x: u8, y: u8 },
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

impl Message {
    //
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let mut messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    // 如果使用 for message in messages
    // 循环的时候，就会将所有数据的所有权拿来，循环之后，messages 就不再可用
    for message in &messages {
        message.call();
    }

    // NOTE：要求变量本身必须是 mut 的
    for message in &mut messages {
        message.call();
    }

    // NOTE: 想要在一个变量上调用 iter_mut方法，这个变量本身也必须是 mut 的
    for message in messages.iter_mut() {
        message.call()
    }

    for message in messages {
        message.call()
    }
}
