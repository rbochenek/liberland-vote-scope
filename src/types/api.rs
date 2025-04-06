use super::*;

/// Account data with optional display name
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiAccount {
    /// The account address
    pub address: String,
    /// Optional identity display name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}

/// Complete election data
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiElectionData {
    /// Block hash where this election data was taken from
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    /// Number of elections commenced so far
    #[serde(rename = "electionRounds")]
    pub election_rounds: u32,
    /// Council configuration
    #[serde(rename = "councilSeats")]
    pub council_seats: ApiCouncilSeats,
    /// Results for all candidates
    #[serde(rename = "finalResults")]
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
    #[serde(rename = "runnersUp")]
    pub runners_up: u32,
}

/// Role in election
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ApiCandidateRole {
    Member,
    RunnerUp,
    NotElected,
}

/// Final result for a candidate
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiCandidateResult {
    /// Candidate account
    pub id: ApiAccount,
    /// Role in election (Member, RunnerUp, NotElected)
    pub role: ApiCandidateRole,
    /// Final Phragmen score
    #[serde(rename = "finalScore")]
    pub final_score: f64,
    /// Initial backing stake
    #[serde(rename = "initialStake")]
    pub initial_stake: u128,
    /// Final applied stake
    #[serde(rename = "finalStake")]
    pub final_stake: u128,
}

/// Basic candidate information
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiCandidate {
    /// Candidate account
    pub id: ApiAccount,
    /// Initial backing stake
    #[serde(rename = "initialStake")]
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
    #[serde(rename = "roundNumber")]
    pub round_number: usize,
    /// Candidate scores for this round
    pub scores: Vec<ApiCandidateScore>,
    /// Vote distribution for this round
    #[serde(rename = "voteDistribution")]
    pub vote_distribution: Vec<ApiVoteDistribution>,
}

/// Candidate score in a specific round
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiCandidateScore {
    /// Candidate account
    pub id: ApiAccount,
    /// Score in this round
    pub score: f64,
    /// Role in final election (for consistent coloring)
    pub role: ApiCandidateRole,
}

/// Vote distribution in a specific round
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiVoteDistribution {
    /// Candidate account
    pub candidate: ApiAccount,
    /// Stake applied to this candidate
    pub stake_applied: u128,
    /// Number of voters supporting this candidate
    #[serde(rename = "voterCount")]
    pub voter_count: usize,
}

/// Helper methods for building API response
impl ApiElectionData {
    /// Build API response from internal data structures
    pub fn build_from(onchain: &OnchainElectionsData, phragmen: &PhragmenOutputs) -> Self {
        // Build council seats
        let council_seats = ApiCouncilSeats {
            members: onchain.desired_members,
            runners_up: onchain.desired_runners_up,
        };

        // Process candidates
        let members: Vec<ApiCandidate> = onchain
            .members
            .iter()
            .map(|seat_holder| ApiCandidate {
                // id: account_with_identity(&seat_holder.who, identity_provider),
                id: ApiAccount::from(&seat_holder.who),
                initial_stake: seat_holder.stake,
            })
            .collect();
        let mut runners_up: Vec<ApiCandidate> = onchain
            .runners_up
            .iter()
            .map(|seat_holder| ApiCandidate {
                id: ApiAccount::from(&seat_holder.who),
                initial_stake: seat_holder.stake,
            })
            .collect();
        let mut other_candidates: Vec<ApiCandidate> = onchain
            .candidates
            .iter()
            .map(|(account_id, stake)| ApiCandidate {
                id: ApiAccount::from(account_id),
                initial_stake: *stake,
            })
            .collect();
        let mut candidates = members;
        candidates.append(&mut runners_up);
        candidates.append(&mut other_candidates);

        // Process voters
        let voters: Vec<ApiVoter> = onchain
            .voting
            .iter()
            .map(|(account_id, voter)| ApiVoter {
                id: ApiAccount::from(account_id),
                stake: voter.stake, // Adjust based on your Voter structure
                votes: voter.votes.iter().map(ApiAccount::from).collect(),
            })
            .collect();

        // Build rounds map
        let mut rounds: Vec<ApiRound> = Vec::new();
        for trace in &phragmen.traces {
            if let PhragmenTrace::RoundStart(round_number, candidates, _) = trace {
                let scores = candidates
                    .clone()
                    .into_iter()
                    .map(|c_ptr| ApiCandidateScore {
                        id: ApiAccount::from(&c_ptr.who),
                        score: c_ptr.score.n() as f64 / c_ptr.score.d() as f64,
                        role: {
                            if c_ptr.elected {
                                if c_ptr.round < onchain.desired_members as usize {
                                    ApiCandidateRole::Member
                                } else {
                                    ApiCandidateRole::RunnerUp
                                }
                            } else {
                                ApiCandidateRole::NotElected
                            }
                        },
                    })
                    .collect();
                rounds.push(ApiRound {
                    round_number: *round_number,
                    scores,
                    vote_distribution: vec![],
                });
            }
        }

        // Build final results
        let mut elected_candidates: Vec<CandidatePtr<AccountId>> = phragmen
            .candidates
            .clone()
            .into_iter()
            .filter(|c_ptr| c_ptr.borrow().elected)
            .collect();
        elected_candidates.sort_by_key(|c_ptr| c_ptr.borrow().round);
        let (elected_members, elected_runners_up) =
            elected_candidates.split_at(council_seats.members as usize);
        let elected_members: Vec<ApiCandidateResult> = elected_members
            .iter()
            .map(|c_ptr| ApiCandidateResult {
                id: ApiAccount::from(&c_ptr.borrow().who),
                role: ApiCandidateRole::Member,
                final_score: c_ptr.borrow().score.n() as f64 / c_ptr.borrow().score.d() as f64,
                initial_stake: c_ptr.borrow().approval_stake,
                final_stake: c_ptr.borrow().backed_stake,
            })
            .collect();
        let mut elected_runners_up: Vec<ApiCandidateResult> = elected_runners_up
            .iter()
            .map(|c_ptr| ApiCandidateResult {
                id: ApiAccount::from(&c_ptr.borrow().who),
                role: ApiCandidateRole::RunnerUp,
                final_score: c_ptr.borrow().score.n() as f64 / c_ptr.borrow().score.d() as f64,
                initial_stake: c_ptr.borrow().approval_stake,
                final_stake: c_ptr.borrow().backed_stake,
            })
            .collect();

        let not_elected_candidates: Vec<CandidatePtr<AccountId>> = phragmen
            .candidates
            .clone()
            .into_iter()
            .filter(|c_ptr| !c_ptr.borrow().elected)
            .collect();
        let mut not_elected_candidates: Vec<ApiCandidateResult> = not_elected_candidates
            .iter()
            .map(|c_ptr| ApiCandidateResult {
                id: ApiAccount::from(&c_ptr.borrow().who),
                role: ApiCandidateRole::NotElected,
                final_score: c_ptr.borrow().score.n() as f64 / c_ptr.borrow().score.d() as f64,
                initial_stake: c_ptr.borrow().approval_stake,
                final_stake: c_ptr.borrow().backed_stake,
            })
            .collect();

        let mut final_results = elected_members;
        final_results.append(&mut elected_runners_up);
        final_results.append(&mut not_elected_candidates);

        Self {
            block_hash: format!("{:?}", onchain.block_hash),
            election_rounds: onchain.election_rounds,
            council_seats,
            final_results,
            candidates,
            voters,
            rounds,
        }
    }
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
