use crate::substrate::runtime_types::pallet_elections_phragmen::{SeatHolder, Voter};
use actix_web::{App, HttpServer, Responder, error, get, web};
use anyhow::{Result, bail};
use clap::Parser;
use serde::{Deserialize, Serialize};
use sp_arithmetic::per_things::Perbill;
use sp_npos_elections::{ElectionResult, PhragmenTrace};
use subxt::{Config, OnlineClient, SubstrateConfig};
use tracing::{Level, event};

mod api;
use api::*;
mod onchain;
use onchain::*;
mod phragmen;
use phragmen::*;
mod types;
use types::*;

#[subxt::subxt(
    runtime_metadata_path = "./artifacts/mainnet.scale",
    derive_for_all_types = "Clone"
)]
pub mod substrate {}

// Command line arguments
#[derive(Clone, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The node to connect to
    #[arg(short, long, default_value = "wss://liberland-rpc.dwellir.com")]
    uri: String,

    /// Increase logging verbosity
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(args.clone()))
            .service(election)
    })
    .workers(3)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
