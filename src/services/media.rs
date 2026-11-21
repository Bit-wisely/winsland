//! Media via Windows GSMTC (GlobalSystemMediaTransportControlsSessionManager events).  (phase 5)
use super::{Service, ServiceResult, Tx};

#[derive(Default)]
pub struct MediaService;

impl Service for MediaService {
    fn name(&self) -> &'static str {
        "media"
    }

    fn start(&mut self, _tx: Tx) -> ServiceResult {
        // TODO(phase 5): subscribe to the OS event source and send Events via _tx.
        Ok(())
    }
}
