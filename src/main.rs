use crate::substrate::runtime_types::pallet_elections_phragmen::{SeatHolder, Voter};
use anyhow::Result;
use clap::Parser;
use sp_arithmetic::per_things::Perbill;
use sp_npos_elections::{ElectionResult, PhragmenTrace};
use std::fs;
use std::path::PathBuf;
use subxt::{SubstrateConfig, config::substrate::AccountId32, utils::H256};
use tracing::{Level, event};

mod markdown;
use markdown::generate_elections_report;
mod onchain;
use onchain::ElectionsDataOnChain;
use onchain::download_onchain_elections_data;
mod phragmen;
use phragmen::*;

#[subxt::subxt(
    runtime_metadata_path = "./artifacts/mainnet.scale",
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

    /// Path to save elections report to
    #[arg(short, long)]
    output: Option<PathBuf>,
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

    // Run Phragmen
    let (phragmen_results, phragmen_tracing) = run_phragmen(phragmen_inputs)?;

    // Generate elections report
    let report = generate_elections_report(&onchain, &phragmen_results, &phragmen_tracing);
    event!(Level::INFO, "{}", report);

    // (optional) Save elections report
    if let Some(path) = args.output {
        event!(
            Level::INFO,
            "Saving elections report to: {}",
            path.display()
        );
        fs::write(path, report).expect("Error saving elections report");
    }

    Ok(())
}
