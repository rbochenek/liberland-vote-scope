use super::*;
use actix_web::Result;

#[get("/council/elections/latest")]
async fn council_elections_latest(
    onchain: web::Data<OnchainDataProvider<SubstrateConfig>>,
) -> Result<impl Responder> {
    let onchain_data = onchain
        .elections_at_blockhash(None)
        .await
        .map_err(|_| error::ErrorBadRequest("Error downloading on-chain elections data"))?;
    let phragmen = simulate_weighted_phragmen_elections(&onchain_data)?;
    let mut result = ApiElectionResults::build_from(&onchain_data, &phragmen);

    // Map addresses to identities
    onchain
        .map_elections_identities(&mut result)
        .await
        .map_err(|_| error::ErrorBadRequest("Error mapping addresses to identites"))?;

    Ok(web::Json(result))
}

#[get("/council/elections/{block_hash}")]
async fn council_elections_at_blockhash(
    path: web::Path<Hash>,
    onchain: web::Data<OnchainDataProvider<SubstrateConfig>>,
) -> Result<impl Responder> {
    let block_hash = path.into_inner();
    let onchain_data = onchain
        .elections_at_blockhash(Some(block_hash))
        .await
        .map_err(|_| error::ErrorBadRequest("Error downloading on-chain elections data"))?;
    let phragmen = simulate_weighted_phragmen_elections(&onchain_data)?;
    let mut result = ApiElectionResults::build_from(&onchain_data, &phragmen);

    // Map addresses to identities
    onchain
        .map_elections_identities(&mut result)
        .await
        .map_err(|_| error::ErrorBadRequest("Error mapping addresses to identites"))?;

    Ok(web::Json(result))
}
