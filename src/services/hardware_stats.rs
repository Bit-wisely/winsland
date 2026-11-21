//! CPU/RAM (sysinfo), GPU (PDH / NVML loaded dynamically). Slow timer, only while expanded.  (phase 8)
use super::{Service, ServiceResult, Tx};

#[derive(Default)]
pub struct HardwareStatsService;

impl Service for HardwareStatsService {
    fn name(&self) -> &'static str {
        "hardware_stats"
    }

    fn start(&mut self, _tx: Tx) -> ServiceResult {
        // TODO(phase 8): subscribe to the OS event source and send Events via _tx.
        Ok(())
    }
}
