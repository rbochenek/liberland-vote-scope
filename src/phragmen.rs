use super::*;
use actix_web::Result;

pub fn simulate_weighted_phragmen_elections(
    onchain_data: &OnchainElectionsData,
) -> Result<PhragmenOutputs> {
    // Convert on-chain data to Phragmen inputs
    let phragmen_inputs = prepare_phragmen_inputs(&onchain_data);

    // Run Phragmen
    run_phragmen(phragmen_inputs)
}

pub fn prepare_phragmen_inputs(onchain: &OnchainElectionsData) -> PhragmenInputs {
    // Collect all Candidates (including current Members and RunnersUp)
    let mut members_and_runnersup = onchain
        .members
        .iter()
        .map(|m| (m.who.clone(), m.deposit))
        .chain(
            onchain
                .runners_up
                .iter()
                .map(|r| (r.who.clone(), r.deposit)),
        )
        .collect::<Vec<_>>();
    let mut candidates_and_deposits = onchain.candidates.clone();
    candidates_and_deposits.append(&mut members_and_runnersup);

    // Calculate number of candidates to elect
    let to_elect = onchain
        .desired_members
        .saturating_add(onchain.desired_runners_up) as usize;

    // Collect Candidate IDs
    let candidate_ids = candidates_and_deposits
        .iter()
        .map(|(x, _)| x)
        .cloned()
        .collect::<Vec<_>>();

    // Collect Voters
    let voters = onchain
        .voting
        .iter()
        .cloned()
        .map(|(voter, Voter { stake, votes, .. })| (voter, stake as u64, votes))
        .collect::<Vec<_>>();

    PhragmenInputs {
        to_elect,
        candidates: candidate_ids,
        voters,
    }
}

pub fn run_phragmen(inputs: PhragmenInputs) -> Result<PhragmenOutputs> {
    match sp_npos_elections::seq_phragmen::<AccountId, Perbill>(
        inputs.to_elect,
        inputs.candidates,
        inputs.voters,
        None,
    ) {
        Ok((result, candidates, traces)) => Ok(PhragmenOutputs {
            result,
            candidates,
            traces,
        }),
        Err(_) => Err(error::ErrorBadRequest("Phragmen internal error")),
    }
}
