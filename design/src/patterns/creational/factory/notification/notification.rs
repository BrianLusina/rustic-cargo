pub trait Notification {
    fn send(&self, message: String);
}
