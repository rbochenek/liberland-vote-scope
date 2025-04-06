use super::*;

pub mod api;

pub type Hash = <SubstrateConfig as Config>::Hash;
pub type AccountId = <SubstrateConfig as Config>::AccountId;

/// Elections data downloaded from the chain
#[derive(Default, Debug)]
pub struct OnchainElectionsData {
    pub block_hash: Hash,
    pub desired_members: u32,
    pub desired_runners_up: u32,
    pub election_rounds: u32,
    pub members: Vec<SeatHolder<AccountId, u128>>,
    pub runners_up: Vec<SeatHolder<AccountId, u128>>,
    pub candidates: Vec<(AccountId, u128)>,
    pub voting: Vec<(AccountId, Voter<AccountId, u128>)>,
}

/// Intermediate representation used by Phragmen
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct PhragmenInputs {
    pub to_elect: usize,
    pub candidates: Vec<AccountId>,
    pub voters: Vec<(AccountId, u64, Vec<AccountId>)>,
}

/// Type returned from hacked sp-npos-elections crate
pub struct PhragmenOutputs {
    pub result: ElectionResult<AccountId, Perbill>,
    pub candidates: Vec<CandidatePtr<AccountId>>,
    pub traces: Vec<PhragmenTrace<AccountId>>,
}
