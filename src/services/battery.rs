//! Battery via RegisterPowerSettingNotification + GetSystemPowerStatus.  (phase 6)
use super::{Service, ServiceResult, Tx};

#[derive(Default)]
pub struct BatteryService;

impl Service for BatteryService {
    fn name(&self) -> &'static str {
        "battery"
    }

    fn start(&mut self, _tx: Tx) -> ServiceResult {
        // TODO(phase 6): subscribe to the OS event source and send Events via _tx.
        Ok(())
    }
}
