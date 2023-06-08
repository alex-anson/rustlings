// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move { x: u32, y: u32 },
    Echo(String),
    ChangeColor(u32, u32, u32),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        // ^ adding a match guard to a pattern
        // downside - the compiler won't try to do exhaustiveness checking
        // 🤨 uhm i feel like this is incorrect ^ .. see below
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let mut experiment: Option<u32> = None;
    experiment = Some(12);

    match experiment {
        Some(a) if a % 2 == 0 => not_nullishh(a),
        Some(a) if a % 2 != 0 => not_nullishh(a),
        // missing match arm: `Some(_)` not covered
        // ^ that's what i'd call exhaustiveness checking...
        Some(a) => not_nullishh(a),
        None => println!("it's the closest thing to null that Rust has"),
    }
}

fn not_nullishh(x: u32) {
    println!("x has a value: {x}")
}
