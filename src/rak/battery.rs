use std::time::Duration;

use anyhow::Result;
use tokio::time::timeout;
use tokio_modbus::prelude::*;
use tokio_serial::SerialStream;

const TTY_PATH: &str = "/dev/ttyUSB0";
const DEVICE_ID: u8 = 0x6e;

// Register address table
const BATTERY_VOLTAGE: u16 = 0x6000;
const BATTERY_HEALTH: u16 = 0x6003;
const REMAINING_CAP: u16 = 0x6005;
const FULL_CHARGE_CAP: u16 = 0x6006;

pub struct BatteryStatus {
    pub battery_voltage: f32,
    pub battery_health_percent: u16,
    pub battery_remaining_percent: f32,
}

impl BatteryStatus {
    pub fn print_kv(&self) {
        println!("battery_voltage={:.2}", self.battery_voltage);
        println!("battery_health_percent={}", self.battery_health_percent);
        println!(
            "battery_remaining_percent={:.2}",
            self.battery_remaining_percent
        );
    }

    pub fn print_collectd(&self, host: &str) {
        println!(
            "PUTVAL {}/battery/gauge-voltage N:{:.2}",
            host, self.battery_voltage
        );
        println!(
            "PUTVAL {}/battery/percent-health N:{}",
            host, self.battery_health_percent
        );
        println!(
            "PUTVAL {}/battery/percent-remaining N:{:.2}",
            host, self.battery_remaining_percent
        )
    }
}

pub async fn battery_status() -> Result<BatteryStatus> {
    let builder = tokio_serial::new(TTY_PATH, 9600);
    let port = SerialStream::open(&builder)?;
    let mut ctx = rtu::attach_slave(port, Slave(DEVICE_ID));
    let to = Duration::from_millis(100);

    Ok(BatteryStatus {
        battery_voltage: {
            let rsp = timeout(to, ctx.read_holding_registers(BATTERY_VOLTAGE, 1)).await???;
            rsp[0] as f32 * 0.01
        },
        battery_health_percent: {
            let rsp = timeout(to, ctx.read_holding_registers(BATTERY_HEALTH, 1)).await???;
            rsp[0]
        },
        battery_remaining_percent: {
            let rsp = timeout(to, ctx.read_holding_registers(FULL_CHARGE_CAP, 1)).await???;
            let full = rsp[0] as f32;
            let rsp = timeout(to, ctx.read_holding_registers(REMAINING_CAP, 1)).await???;
            let remaining = rsp[0] as f32;

            remaining / full * 100.0
        },
    })
}
