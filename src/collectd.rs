use std::env;
use std::time::Duration;

use crate::rak;
use anyhow::Result;
use tokio::time::sleep;

pub async fn run() -> Result<()> {
    let host = env::var("COLLECTD_HOSTNAME").unwrap_or_default();
    let interval = env::var("COLLECTD_INTERVAL").unwrap_or_default();

    if host.is_empty() || interval.is_empty() {
        return Ok(());
    }

    let interval: f64 = interval.parse()?;
    let interval = Duration::from_secs(interval as u64);

    loop {
        if let Ok(v) = rak::battery::battery_status().await {
            v.print_collectd(&host);
        }

        sleep(interval).await;
    }
}
