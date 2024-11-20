enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}
#[cfg(test)]
mod tests {
    use crate::message::Message;

    #[test]
    fn test_ip_address() {
        let m = Message::Write(String::from("Hello"));
        m.call();
    }
}
