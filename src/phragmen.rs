use super::ElectionsDataOnChain;
use crate::substrate::runtime_types::pallet_elections_phragmen::Voter;
use anyhow::{Result, bail};
use sp_arithmetic::per_things::Perbill;
use sp_npos_elections::ElectionResult;
use subxt::utils::AccountId32;

#[derive(Default, Debug)]
pub struct PhragmenInputs {
    pub to_elect: usize,
    pub candidates: Vec<AccountId32>,
    pub voters: Vec<(AccountId32, u64, Vec<AccountId32>)>,
}

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
    // TODO: Add CLI argument for `MaxMembers` and `MaxRunnersUp`
    let to_elect = 14;

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

pub fn run_phragmen(inputs: PhragmenInputs) -> Result<ElectionResult<AccountId32, Perbill>> {
    match sp_npos_elections::seq_phragmen::<AccountId32, Perbill>(
        inputs.to_elect,
        inputs.candidates,
        inputs.voters,
        None,
    ) {
        Ok(results) => Ok(results),
        Err(_) => {
            bail!("seq_phragmen() normalization error");
        }
    }
}
