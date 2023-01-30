use btleplug::api::{Central, Manager as _, ScanFilter};
use btleplug::platform::{Manager, Peripheral};
use std::error::Error;

pub struct BlueManager {}

impl BlueManager {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn list(&mut self) -> Result<Vec<Peripheral>, Box<dyn Error>> {
        let manager = Manager::new().await?;

        let central = manager
            .adapters()
            .await
            .expect("Unable to fetch adapter list.")
            .into_iter()
            .next()
            .expect("Unable to find adapters.");

        central.start_scan(ScanFilter::default()).await?;

        let list = central.peripherals().await?;

        Ok(list)
    }
}
