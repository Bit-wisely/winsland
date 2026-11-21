//! Volume via IAudioEndpointVolume callbacks.  (phase 6)
use super::{Service, ServiceResult, Tx};

#[derive(Default)]
pub struct VolumeService;

impl Service for VolumeService {
    fn name(&self) -> &'static str {
        "volume"
    }

    fn start(&mut self, _tx: Tx) -> ServiceResult {
        // TODO(phase 6): subscribe to the OS event source and send Events via _tx.
        Ok(())
    }
}
