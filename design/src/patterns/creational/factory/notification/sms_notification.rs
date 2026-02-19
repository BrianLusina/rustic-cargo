use crate::patterns::creational::factory::notification::notification::Notification;

pub struct SmsNotification {}

impl SmsNotification {
    pub(crate) fn new() -> Box<dyn Notification> {
        todo!()
    }
}

impl Notification for SmsNotification {
    fn send(&self, message: String) {
        todo!()
    }
}
