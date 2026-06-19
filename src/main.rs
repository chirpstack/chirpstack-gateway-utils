use anyhow::Result;
use clap::{Parser, Subcommand};

mod rak;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Vendors,
}

#[derive(Subcommand)]
enum Vendors {
    /// Rak
    Rak {
        #[command(subcommand)]
        component: RakComponent,
    },
}

#[derive(Subcommand)]
enum RakComponent {
    /// Watchdog.
    FeedWatchdog {},

    /// BatteryStatus
    PrintBatteryStatus {},
}

#[derive(Subcommand)]
enum RakWatchdog {
    /// Feed watchdog
    Feed {},
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Vendors::Rak { component } => match component {
            RakComponent::FeedWatchdog {} => rak::watchdog::feed().await?,
            RakComponent::PrintBatteryStatus {} => rak::battery::battery_status().await?.print(),
        },
    }

    Ok(())
}
