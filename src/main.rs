use anyhow::Result;
use clap::Parser;
use sp_npos_elections::PhragmenTrace;
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
        .without_time()
        .with_target(false)
        .with_level(false)
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
    let (phragmen_results, phragmen_tracing) = run_phragmen(phragmen_inputs)?;

    // Show Phragmen results
    event!(Level::INFO, "--- Phragmen results ---");
    for winner in &phragmen_results.winners {
        event!(Level::INFO, "{} {}", winner.0.to_string(), winner.1);
    }

    // Show Phragmen tracing
    event!(Level::INFO, "--- Phragmen traces ---");
    for trace in &phragmen_tracing {
        match trace {
            PhragmenTrace::Start => event!(Level::INFO, "Phragmen started"),
            PhragmenTrace::Finish => event!(Level::INFO, "Phragmen finished"),
            PhragmenTrace::ToElect(to_elect) => {
                event!(Level::INFO, "Candidates to elect: {to_elect}")
            }
            PhragmenTrace::RoundStart(round_no, candidates, voters) => {
                event!(
                    Level::INFO,
                    "Election round {round_no} [candidates: {}, voters: {}]",
                    candidates.len(),
                    voters.len()
                )
            }
            PhragmenTrace::CandidateScoresCalculated(_candidates) => {
                event!(Level::INFO, "Candidate scores calculated")
            }
            PhragmenTrace::CandidateScoresUpdatedByVoters(_candidates) => {
                event!(Level::INFO, "Candidate scores updated by voters")
            }
            PhragmenTrace::CandidateElected(winner) => {
                event!(Level::INFO, "Candidate elected: {}", winner.who.to_string())
            }
            PhragmenTrace::VoterEdgeUpdated(edge) => {
                event!(
                    Level::INFO,
                    "Voter edge load updated {} => {} [{:?} => {:?}]",
                    edge.voter.to_string(),
                    edge.candidate.to_string(),
                    edge.load,
                    edge.new_load,
                )
            }
            PhragmenTrace::VoterLoadUpdated(load) => {
                event!(
                    Level::INFO,
                    "Voter load updated {} [{:?} => {:?}]",
                    load.who.to_string(),
                    load.load,
                    load.new_load
                );
            }
            PhragmenTrace::CandidateScoreUpdated(update) => {
                event!(
                    Level::INFO,
                    "Candidate score updated {} [{:?} => {:?}]",
                    update.who.to_string(),
                    update.score,
                    update.new_score
                );
            }
            PhragmenTrace::CandidateScoreUpdatedByVoter(update) => {
                event!(
                    Level::INFO,
                    "Candidate score updated by voter {} => {} [{:?} => {:?}]",
                    update.voter.to_string(),
                    update.candidate.to_string(),
                    update.score,
                    update.new_score
                );
            }
        }
    }

    Ok(())
}
