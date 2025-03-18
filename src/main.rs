use anyhow::Result;
use clap::Parser;
use subxt::SubstrateConfig;
use tracing::{Level, event};

mod onchain;
pub use onchain::ElectionsDataOnChain;
use onchain::download_onchain_elections_data;
mod phragmen;
use phragmen::*;

#[subxt::subxt(
    runtime_metadata_path = "./artifacts/dev.scale",
    derive_for_all_types = "Clone"
)]
pub mod substrate {}

// Command line arguments
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The node to connect to
    #[arg(short, long, default_value = "wss://liberland-rpc.dwellir.com")]
    uri: String,

    /// Fetch elections data at given block hash
    #[arg(short, long)]
    at: Option<<SubstrateConfig as subxt::Config>::Hash>,

    /// Increase logging verbosity
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Set up tracing
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(if args.verbose {
            Level::DEBUG
        } else {
            Level::INFO
        })
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Default tracing subscriber error");

    // Download on-chain Elections data
    let onchain = download_onchain_elections_data(&args).await?;

    // Prepare Phragmen inputs
    let phragmen_inputs = prepare_phragmen_inputs(&onchain)?;

    event!(Level::DEBUG, "--- Candidate IDs ---");
    for c in &phragmen_inputs.candidates {
        event!(Level::DEBUG, "{}", c.to_string());
    }

    event!(Level::DEBUG, "--- Voters ---");
    for v in &phragmen_inputs.voters {
        event!(Level::DEBUG, "{} {}", v.0.to_string(), v.1);
    }

    // Run Phragmen
    let phragmen_results = run_phragmen(phragmen_inputs)?;
    event!(Level::INFO, "--- Phragmen output ---");
    for winner in &phragmen_results.winners {
        event!(Level::INFO, "{} {}", winner.0.to_string(), winner.1);
    }

    Ok(())
}
