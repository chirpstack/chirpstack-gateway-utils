use anyhow::Result;
use tokio_modbus::prelude::*;
use tokio_serial::SerialStream;

const TTY_PATH: &str = "/dev/ttyUSB0";
const DEVICE_ID: u8 = 0xc8;

const WATCHDOG_FEED: u16 = 0x0001;

pub async fn feed() -> Result<()> {
    let builder = tokio_serial::new(TTY_PATH, 9600);
    let port = SerialStream::open(&builder)?;
    let mut ctx = rtu::attach_slave(port, Slave(DEVICE_ID));

    ctx.write_single_register(WATCHDOG_FEED, 10).await??;

    Ok(())
}
