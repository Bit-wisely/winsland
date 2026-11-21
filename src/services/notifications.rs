//! Notifications via UserNotificationListener (needs MSIX package identity).  (phase 7)
use super::{Service, ServiceResult, Tx};

#[derive(Default)]
pub struct NotificationService;

impl Service for NotificationService {
    fn name(&self) -> &'static str {
        "notifications"
    }

    fn start(&mut self, _tx: Tx) -> ServiceResult {
        // TODO(phase 7): subscribe to the OS event source and send Events via _tx.
        Ok(())
    }
}
