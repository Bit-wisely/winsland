//! Optional weather (Open-Meteo via ureq). Off by default.  (phase 9)
use super::{Service, ServiceResult, Tx};

#[derive(Default)]
pub struct WeatherService;

impl Service for WeatherService {
    fn name(&self) -> &'static str {
        "weather"
    }

    fn start(&mut self, _tx: Tx) -> ServiceResult {
        // TODO(phase 9): subscribe to the OS event source and send Events via _tx.
        Ok(())
    }
}
