// enums1.rs
//
// No hints this time! ;)

// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo(String),
    Move(i32,i32),
    ChangeColor(u8,u8,u8)
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo("hello world".to_string()));
    println!("{:?}", Message::Move(10,20));
    println!("{:?}", Message::ChangeColor(255,0,0));
}
