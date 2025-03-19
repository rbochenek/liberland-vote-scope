use crate::{Candidate, Rational128, Voter};

pub type Candidates<AccountId> = Vec<Candidate<AccountId>>;
pub type Voters<AccountId> = Vec<Voter<AccountId>>;

#[derive(Clone)]
pub struct LoadUpdate<AccountId> {
    pub who: AccountId,
    pub load: Rational128,
    pub new_load: Rational128,
}

#[derive(Clone)]
pub struct EdgeLoadUpdate<AccountId> {
    pub voter: AccountId,
    pub candidate: AccountId,
    pub load: Rational128,
    pub new_load: Rational128,
}

#[derive(Clone)]
pub struct CandidateScoreUpdate<AccountId> {
    pub who: AccountId,
    pub score: Rational128,
    pub new_score: Rational128,
}

#[derive(Clone)]
pub struct CandidateScoreUpdateByVoter<AccountId> {
    pub voter: AccountId,
    pub candidate: AccountId,
    pub score: Rational128,
    pub new_score: Rational128,
}

/// Trace of internal step inside Phragmen.
#[derive(Clone)]
pub enum PhragmenTrace<AccountId> {
    Start,
    Finish,
    ToElect(usize),
    RoundStart(usize, Candidates<AccountId>, Voters<AccountId>),
    ComputeCandidateScores,
    IncCandidateScoresByVoters,
    CandidateScoresCalculated(Candidates<AccountId>),
    CandidateScoresUpdatedByVoters(Candidates<AccountId>),
    CandidateElected(Candidate<AccountId>),
    VoterEdgeUpdated(EdgeLoadUpdate<AccountId>),
    VoterLoadUpdated(LoadUpdate<AccountId>),
    CandidateScoreUpdated(CandidateScoreUpdate<AccountId>),
    CandidateScoreUpdatedByVoter(CandidateScoreUpdateByVoter<AccountId>),
}
