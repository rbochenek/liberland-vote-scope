use super::*;

pub type Hash = <SubstrateConfig as Config>::Hash;
pub type AccountId = <SubstrateConfig as Config>::AccountId;

#[derive(Default, Debug)]
pub struct ElectionsDataOnChain {
    pub block_hash: Hash,
    pub desired_members: u32,
    pub desired_runners_up: u32,
    pub election_rounds: u32,
    pub members: Vec<SeatHolder<AccountId, u128>>,
    pub runners_up: Vec<SeatHolder<AccountId, u128>>,
    pub candidates: Vec<(AccountId, u128)>,
    pub voting: Vec<(AccountId, Voter<AccountId, u128>)>,
}

#[derive(Default, Debug)]
pub struct PhragmenInputs {
    pub to_elect: usize,
    pub candidates: Vec<AccountId>,
    pub voters: Vec<(AccountId, u64, Vec<AccountId>)>,
}
