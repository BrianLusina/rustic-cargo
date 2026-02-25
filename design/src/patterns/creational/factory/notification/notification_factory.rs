use crate::patterns::creational::factory::notification::email_notification::EmailNotification;
use crate::patterns::creational::factory::notification::notification::Notification;
use crate::patterns::creational::factory::notification::sms_notification::SmsNotification;

struct NotificationFactory {}

impl NotificationFactory {
    pub fn create_notification(&self, notification_type: String) -> Box<dyn Notification> {
        match notification_type.as_str() {
            "email" => EmailNotification::new(),
            "sms" => SmsNotification::new(),
            _ => panic!("Invalid notification type"),
        }
    }
}
