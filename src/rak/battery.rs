use std::env;

use anyhow::Result;
use tokio_modbus::prelude::*;
use tokio_serial::SerialStream;

const TTY_PATH: &str = "/dev/ttyUSB0";
const DEVICE_ID: u8 = 0x6e;

// Register address table
const BATTERY_VOLTAGE: u16 = 0x6000;
const BATTERY_CURRENT: u16 = 0x6001;
const BATTERY_HEALTH: u16 = 0x6003;
const REMAINING_CAP: u16 = 0x6005;
const FULL_CHARGE_CAP: u16 = 0x6006;

pub struct BatteryStatus {
    pub battery_voltage: f32,
    pub battery_current: f32,
    pub battery_health_percentage: u16,
    pub battery_remaining_percentage: f32,
}

impl BatteryStatus {
    pub fn print(&self) {
        let host = env::var("COLLECTD_HOSTNAME").unwrap_or_default();
        if host.is_empty() {
            println!("battery_voltage={:.2}", self.battery_voltage);
            println!("battery_current={:.2}", self.battery_current);
            println!("battery_health_percent={}", self.battery_health_percentage);
            println!(
                "battery_remaining_percent={:.2}",
                self.battery_remaining_percentage
            );
        } else {
            println!(
                "PUTVAL {}/battery/gauge-voltage N:{:.2}",
                host, self.battery_voltage
            );
            println!(
                "PUTVAL {}/battery/gauge-current N:{:.2}",
                host, self.battery_current,
            );
            println!(
                "PUTVAL {}/battery/percentage-health N:{}",
                host, self.battery_health_percentage
            );
            println!(
                "PUTVAL {}/battery/percentage-remaining N:{:.2}",
                host, self.battery_remaining_percentage
            )
        }
    }
}

pub async fn battery_status() -> Result<BatteryStatus> {
    let builder = tokio_serial::new(TTY_PATH, 9600);
    let port = SerialStream::open(&builder)?;
    let mut ctx = rtu::attach_slave(port, Slave(DEVICE_ID));

    Ok(BatteryStatus {
        battery_voltage: {
            let rsp = ctx.read_holding_registers(BATTERY_VOLTAGE, 1).await??;
            rsp[0] as f32 * 0.01
        },
        battery_current: {
            let rsp = ctx.read_holding_registers(BATTERY_CURRENT, 1).await??;
            (rsp[0] as i16) as f32 * 0.01
        },
        battery_health_percentage: {
            let rsp = ctx.read_holding_registers(BATTERY_HEALTH, 1).await??;
            rsp[0]
        },
        battery_remaining_percentage: {
            let rsp = ctx.read_holding_registers(FULL_CHARGE_CAP, 1).await??;
            let full = rsp[0] as f32;
            let rsp = ctx.read_holding_registers(REMAINING_CAP, 1).await??;
            let remaining = rsp[0] as f32;

            remaining / full * 100.0
        },
    })
}
