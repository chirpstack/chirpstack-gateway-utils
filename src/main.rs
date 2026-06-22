use anyhow::Result;
use clap::{Parser, Subcommand};

mod collectd;
mod rak;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Sub,
}

#[derive(Subcommand)]
enum Sub {
    /// Rak
    Rak {
        #[command(subcommand)]
        component: RakComponent,
    },

    /// Expose metrics to Collectd.
    Collectd {},
}

#[derive(Subcommand)]
enum RakComponent {
    /// Watchdog.
    FeedWatchdog {},

    /// Print the battery status (key=value)
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
        Sub::Rak { component } => match component {
            RakComponent::FeedWatchdog {} => rak::watchdog::feed().await?,
            RakComponent::PrintBatteryStatus {} => rak::battery::battery_status().await?.print_kv(),
        },
        Sub::Collectd {} => {
            collectd::run().await?;
        }
    }

    Ok(())
}
