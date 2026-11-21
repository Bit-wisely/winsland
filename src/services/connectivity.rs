//! Wi-Fi / Bluetooth via NetworkInformation and Windows.Devices.Radios events.  (phase 6)
use super::{Service, ServiceResult, Tx};

#[derive(Default)]
pub struct ConnectivityService;

impl Service for ConnectivityService {
    fn name(&self) -> &'static str {
        "connectivity"
    }

    fn start(&mut self, _tx: Tx) -> ServiceResult {
        // TODO(phase 6): subscribe to the OS event source and send Events via _tx.
        Ok(())
    }
}
