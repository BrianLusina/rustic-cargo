use crate::patterns::creational::factory::notification::notification::Notification;

pub struct EmailNotification {}

impl EmailNotification {
    pub(crate) fn new() -> Box<dyn Notification> {
        todo!()
    }
}

impl Notification for EmailNotification {
    fn send(&self, message: String) {
        todo!()
    }
}
