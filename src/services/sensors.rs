//! Optional temperature/fan sidecar client over a named pipe. Off by default.  (phase 8)
use super::{Service, ServiceResult, Tx};

#[derive(Default)]
pub struct SensorsService;

impl Service for SensorsService {
    fn name(&self) -> &'static str {
        "sensors"
    }

    fn start(&mut self, _tx: Tx) -> ServiceResult {
        // TODO(phase 8): subscribe to the OS event source and send Events via _tx.
        Ok(())
    }
}
