enum IPAddrKind {
    V4,
    V6,
}

enum IPAddr {
    //V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call (&self) {}
}

fn main() {
    let IPv4 = IPAddrKind::V4;
    let IPv6 = IPAddrKind::V6;
    
    //let home = IPAddr::V4(String::from("127.0.0.1"));
    let home = IPAddr::V4(127, 0, 0, 1);
    let loopback = IPAddr::V6(String::from("::1"));
    
    let msg = Message::Write(String::from("Hello"));
    msg.call();
    println!("Hello, world!");
}
