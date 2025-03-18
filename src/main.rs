use anyhow::Result;
use clap::Parser;
use subxt::{OnlineClient, SubstrateConfig};
use tracing::{Level, event};

#[subxt::subxt(runtime_metadata_path = "./artifacts/dev.scale")]
pub mod substrate {}

// Command line arguments
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The node to connect to
    // #[arg(short, long, default_value = "ws://localhost:9944")]
    #[arg(short, long, default_value = "wss://liberland-rpc.dwellir.com")]
    uri: String,

    /// Fetch elections data at given block hash
    #[arg(short, long)]
    at: Option<<SubstrateConfig as subxt::Config>::Hash>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Set up tracing
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Default tracing subscriber error");

    // Connect to node
    event!(Level::INFO, "Connecting to {}", &args.uri);
    let api = OnlineClient::<SubstrateConfig>::from_url(&args.uri).await?;

    // Prepare block hash to operate on
    let block_hash = match args.at {
        Some(hash) => hash,
        None => {
            let latest_block_hash = api.blocks().at_latest().await?.hash();
            event!(
                Level::DEBUG,
                "Block hash not provided. Using latest block hash: {:?}",
                latest_block_hash
            );
            latest_block_hash
        }
    };
    let storage = api.storage().at(block_hash);

    // Fetch number of election rounds
    let election_rounds = storage
        .fetch(&substrate::storage().elections().election_rounds())
        .await?
        .expect("ElectionRounds not found in storage");
    event!(Level::INFO, "ElectionRounds = {}", election_rounds);

    // Fetch Members
    event!(Level::INFO, "--- Members ---");
    let members = storage
        .fetch(&substrate::storage().elections().members())
        .await?
        .expect("Members not found in storage");
    for member in &members {
        event!(
            Level::INFO,
            "{} [stake: {} deposit: {}]",
            member.who.to_string(),
            member.stake,
            member.deposit
        );
    }

    // Fetch RunnersUp
    event!(Level::INFO, "--- RunnersUp ---");
    let runners_up = storage
        .fetch(&substrate::storage().elections().runners_up())
        .await?
        .expect("RunnersUp not found in storage");
    for runner in &runners_up {
        event!(
            Level::INFO,
            "{} [stake: {} deposit: {}]",
            runner.who.to_string(),
            runner.stake,
            runner.deposit
        );
    }

    // Fetch Candidates
    event!(Level::INFO, "--- Candidates ---");
    let candidates = storage
        .fetch(&substrate::storage().elections().candidates())
        .await?;
    match candidates {
        Some(candidates) => {
            for candidate in &candidates {
                event!(
                    Level::INFO,
                    "{} [deposit: {}]",
                    candidate.0.to_string(),
                    candidate.1
                );
            }
        }
        None => event!(Level::INFO, "No Candidates found."),
    }

    // Fetch Voting
    event!(Level::INFO, "--- Voting ---");
    let mut voting = storage
        .iter(substrate::storage().elections().voting_iter())
        .await?;
    while let Some(Ok(kv)) = voting.next().await {
        let voter = kv.value;
        event!(
            Level::INFO,
            "key_bytes len = {}",
            kv.key_bytes.clone().len()
        );
        let who: [u8; 32] = kv.key_bytes[40..].try_into().unwrap();
        let voter_account = <SubstrateConfig as subxt::Config>::AccountId::from(who);
        event!(Level::INFO, "{}", voter_account.to_string());
    }

    Ok(())
}
