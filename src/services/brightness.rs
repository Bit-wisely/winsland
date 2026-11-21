//! Brightness: WMI for laptop panels, DDC/CI (dxva2) for external monitors.  (phase 6)
use super::{Service, ServiceResult, Tx};

#[derive(Default)]
pub struct BrightnessService;

impl Service for BrightnessService {
    fn name(&self) -> &'static str {
        "brightness"
    }

    fn start(&mut self, _tx: Tx) -> ServiceResult {
        // TODO(phase 6): subscribe to the OS event source and send Events via _tx.
        Ok(())
    }
}
