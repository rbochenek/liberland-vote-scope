use super::*;
use actix_web::Result;

#[get("/elections/latest")]
async fn election_latest(args: web::Data<Args>) -> Result<impl Responder, error::Error> {
    let results = fetch_election_results(None, &args).await?;

    Ok(web::Json(results))
}

#[get("/elections/{block_hash}")]
async fn election(
    path: web::Path<Hash>,
    args: web::Data<Args>,
) -> Result<impl Responder, error::Error> {
    let block_hash = path.into_inner();

    let results = fetch_election_results(Some(block_hash), &args).await?;

    Ok(web::Json(results))
}

async fn fetch_election_results(
    block_hash: Option<Hash>,
    args: &Args,
) -> Result<ApiElectionResults> {
    // Download on-chain data
    let onchain_data = download_onchain_elections_data(block_hash, args)
        .await
        .map_err(|_| error::ErrorBadRequest("Error downloading on-chain data"))?;

    // Convert on-chain data to Phragmen inputs
    let phragmen_inputs = prepare_phragmen_inputs(&onchain_data)
        .map_err(|_| error::ErrorBadRequest("Error preparing Phragmen inputs"))?;

    // Run Phragmen
    let (election_results, candidates, phragmen_traces) = run_phragmen(phragmen_inputs.clone())
        .map_err(|_| error::ErrorBadRequest("Phragmen computation error"))?;

    // Generate JSON response with election data
    let results = ApiElectionResults::build_from(
        &onchain_data,
        &election_results,
        &candidates,
        &phragmen_traces,
    );

    Ok(results)
}
