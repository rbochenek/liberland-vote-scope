use super::*;

pub type Hash = <SubstrateConfig as Config>::Hash;
pub type AccountId = <SubstrateConfig as Config>::AccountId;

/// Elections data downloaded from blockchain
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

/// Intermediate representation used by Phragmen
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct PhragmenInputs {
    pub to_elect: usize,
    pub candidates: Vec<AccountId>,
    pub voters: Vec<(AccountId, u64, Vec<AccountId>)>,
}

/// API response for election results
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiElectionResults {
    /// Block hash where this election data was taken from
    pub block_hash: String,
    /// Complete election data
    pub election_data: ApiElectionData,
}

/// Account data with optional display name
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiAccount {
    /// The account address
    pub address: String,
    /// Optional identity display name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

/// Complete election data
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiElectionData {
    /// Council configuration
    pub council_seats: ApiCouncilSeats,
    /// Results for all candidates
    pub final_results: Vec<ApiCandidateResult>,
    /// Candidates with initial stake
    pub candidates: Vec<ApiCandidate>,
    /// Voters with their stakes and votes
    pub voters: Vec<ApiVoter>,
    /// Detailed rounds of the Phragmen algorithm
    pub rounds: Vec<ApiRound>,
}

/// Council seats configuration
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiCouncilSeats {
    /// Number of members
    pub members: u32,
    /// Number of runners up
    pub runners_up: u32,
}

/// Final result for a candidate
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiCandidateResult {
    /// Candidate account
    pub candidate: ApiAccount,
    /// Role in election (Member, RunnerUp, NotElected)
    pub role: String,
    /// Final Phragmen score
    pub final_score: f64,
    /// Initial backing stake
    pub initial_stake: u128,
    /// Final applied stake
    pub final_stake: u128,
}

/// Basic candidate information
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiCandidate {
    /// Candidate account
    pub id: ApiAccount,
    /// Initial backing stake
    pub initial_stake: u128,
}

/// Voter information
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiVoter {
    /// Voter account
    pub id: ApiAccount,
    /// Total stake
    pub stake: u128,
    /// Votes cast for candidates
    pub votes: Vec<ApiAccount>,
}

/// Information about a specific election round
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiRound {
    /// Round number (1-based)
    pub round_number: usize,
    /// Candidate scores for this round
    pub scores: Vec<ApiCandidateScore>,
    /// Vote distribution for this round
    pub vote_distribution: Vec<ApiVoteDistribution>,
}

/// Candidate score in a specific round
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiCandidateScore {
    /// Candidate account
    pub candidate: ApiAccount,
    /// Score in this round
    pub score: f64,
    /// Role in final election (for consistent coloring)
    pub role: String,
}

/// Vote distribution in a specific round
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiVoteDistribution {
    /// Candidate account
    pub candidate: ApiAccount,
    /// Stake applied to this candidate
    pub stake_applied: u128,
    /// Number of voters supporting this candidate
    pub voter_count: usize,
}

/// Helper methods for building API response
impl ApiElectionResults {
    /// Build API response from internal data structures
    pub fn build_from(
        on_chain: &ElectionsDataOnChain,
        result: &ElectionResult<AccountId, Perbill>,
        traces: &Vec<PhragmenTrace<AccountId>>,
        identity_provider: &impl IdentityProvider,
    ) -> Self {
        // Implementation will be provided
        unimplemented!()
    }
}

/// Trait for identity provider implementations
pub trait IdentityProvider {
    /// Get display name for an account, if available
    fn get_display_name(&self, account: &AccountId) -> Option<String>;
}

/// Helper for converting AccountId to ApiAccount
impl From<&AccountId> for ApiAccount {
    fn from(account: &AccountId) -> Self {
        // Basic conversion without display name
        Self {
            address: account.to_string(),
            display_name: None,
        }
    }
}

/// Helper for converting AccountId to ApiAccount with identity lookup
pub fn account_with_identity(
    account: &AccountId,
    identity_provider: &impl IdentityProvider,
) -> ApiAccount {
    ApiAccount {
        address: account.to_string(),
        display_name: identity_provider.get_display_name(account),
    }
}
