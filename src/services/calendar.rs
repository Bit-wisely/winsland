//! Calendar: ICS offline-first, optional Appointments API.  (phase 9)
use super::{Service, ServiceResult, Tx};

#[derive(Default)]
pub struct CalendarService;

impl Service for CalendarService {
    fn name(&self) -> &'static str {
        "calendar"
    }

    fn start(&mut self, _tx: Tx) -> ServiceResult {
        // TODO(phase 9): subscribe to the OS event source and send Events via _tx.
        Ok(())
    }
}
