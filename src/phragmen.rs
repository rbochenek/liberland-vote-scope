use super::*;

pub fn prepare_phragmen_inputs(onchain: &ElectionsDataOnChain) -> Result<PhragmenInputs> {
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

    Ok(PhragmenInputs {
        to_elect,
        candidates: candidate_ids,
        voters,
    })
}

pub fn run_phragmen(
    inputs: PhragmenInputs,
) -> Result<(
    ElectionResult<AccountId, Perbill>,
    Vec<PhragmenTrace<AccountId>>,
)> {
    match sp_npos_elections::seq_phragmen::<AccountId, Perbill>(
        inputs.to_elect,
        inputs.candidates,
        inputs.voters,
        None,
    ) {
        Ok((results, tracing)) => Ok((results, tracing)),
        Err(_) => {
            bail!("seq_phragmen() normalization error");
        }
    }
}
