use super::*;
use actix_web::Result;

#[get("/elections/{block_hash}")]
async fn election(
    path: web::Path<Hash>,
    args: web::Data<Args>,
) -> Result<impl Responder, error::Error> {
    let block_hash = path.into_inner();

    // Download on-chain data
    let onchain_data = download_onchain_elections_data(Some(block_hash), &args)
        .await
        .map_err(|_| error::ErrorBadRequest("Error downloading on-chain data"))?;

    // Convert on-chain data to Phragmen inputs
    let phragmen_inputs = prepare_phragmen_inputs(&onchain_data)
        .map_err(|_| error::ErrorBadRequest("Error preparing Phragmen inputs"))?;

    // Run Phragmen
    let (_election_results, _phragmen_traces) = run_phragmen(phragmen_inputs.clone())
        .map_err(|_| error::ErrorBadRequest("Phragmen computation error"))?;

    Ok(web::Json(phragmen_inputs))
}
