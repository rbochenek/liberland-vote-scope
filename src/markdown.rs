use super::*;

pub fn generate_elections_report(
    onchain: &ElectionsDataOnChain,
    results: &ElectionResult<AccountId, Perbill>,
    tracing: &Vec<PhragmenTrace<AccountId>>,
) -> String {
    let mut out = String::new();

    out += &generate_elections_report_inputs(onchain);
    out += &generate_elections_report_phragmen_traces(tracing);
    out += &generate_elections_report_outcome(results);

    out
}

fn generate_elections_report_inputs(onchain: &ElectionsDataOnChain) -> String {
    let mut out = String::new();

    out += "# Before elections  \n  \n";
    out += &format!(
        "On-chain data fetched from block hash {:#?}  \n",
        onchain.block_hash
    );
    out += &format!("Elections so far: {}  \n", onchain.election_rounds);
    out += "  \n";
    out += "<details open>\n<summary>Members</summary>  \n";
    out += "  \n";
    out += &table_seatholders(&onchain.members);
    out += &format!("Total count: {}  \n  \n", onchain.members.len());
    out += "</details>";

    out += "  \n";
    out += "<details open>\n<summary>Runners up</summary>  \n";
    out += "  \n";
    out += &table_seatholders(&onchain.runners_up);
    out += &format!("Total count: {}  \n  \n", onchain.runners_up.len());
    out += "</details>  \n";
    out += "  \n";

    out += "  \n";
    out += "<details open>\n<summary>Candidates</summary>  \n";
    out += "  \n";
    out += &table_candidates(&onchain.candidates);
    out += &format!("Total count: {}  \n  \n", onchain.candidates.len());
    out += "</details>  \n";
    out += "  \n";

    out
}

fn generate_elections_report_phragmen_traces(tracing: &Vec<PhragmenTrace<AccountId>>) -> String {
    let mut out = String::new();

    for trace in tracing {
        match trace {
            PhragmenTrace::Start => out += "# Phragmen traces  \n  \n",
            PhragmenTrace::ToElect(to_elect) => {
                out += &format!("Candidates to elect: {}  \n  \n", to_elect)
            }
            PhragmenTrace::RoundStart(round_no, _, _) => {
                out += &format!("## Round {}  \n", round_no)
            }
            PhragmenTrace::ComputeCandidateScores => {
                out += "<details><summary>Calculate candidate scores (1 / approval_stake)</summary>  \n  \n"
            }
            PhragmenTrace::CandidateScoreUpdated(update) => {
                out += &format!(
                    "Candidate {} score updated to {:?}  \n",
                    update.who.to_string(),
                    update.new_score
                );
            }
            PhragmenTrace::CandidateScoresCalculated(_candidates) => out += "</details>  \n",
            PhragmenTrace::IncCandidateScoresByVoters => {
                out += "<details><summary>Increase candidate scores by voters</summary>  \n  \n"
            }
            PhragmenTrace::CandidateScoreUpdatedByVoter(update) => {
                out += &format!(
                    "Voter {} updated candidate {} score to {:?}  \n",
                    update.voter.to_string(),
                    update.candidate.to_string(),
                    update.new_score
                );
            }
            PhragmenTrace::CandidateScoresUpdatedByVoters(_candidates) => out += "</details>  \n",
            PhragmenTrace::CandidateElected(candidate) => {
                out += "<details open><summary>Candidate elected</summary>  \n  \n";
                out += &format!(
                    "Account: {}  \nScore: {:?}  \nApproval stake: {}  \n",
                    candidate.who.to_string(),
                    candidate.score,
                    candidate.approval_stake,
                );
                out += "</details>  \n  \n";
            }
            PhragmenTrace::Finish => out += "  \n  \n",
            _ => {}
        }
    }

    out
}
fn generate_elections_report_outcome(results: &ElectionResult<AccountId, Perbill>) -> String {
    let mut out = String::new();

    out += "# Election results  \n  \n";
    out += "  \n";
    out += "<details open>\n<summary>Winners</summary>  \n";
    out += "  \n";
    out += &table_winners(results);
    out += &format!("Total count: {}  \n  \n", results.winners.len());
    out += "</details>  \n";
    out += "  \n";

    out
}

fn table_seatholders(seatholders: &Vec<SeatHolder<AccountId, u128>>) -> String {
    let mut out = String::new();

    out += "  \n";
    out += "| Account | Stake | Deposit |  \n";
    out += "| --- | --- | --- |  \n";
    for holder in seatholders {
        out += &format!(
            "| {} | {} | {} |  \n",
            holder.who.to_string(),
            holder.stake,
            holder.deposit
        );
    }
    out += "  \n";

    out
}

fn table_candidates(candidates: &Vec<(AccountId, u128)>) -> String {
    let mut out = String::new();

    out += "  \n";
    out += "| Account | Deposit |  \n";
    out += "| --- | --- |  \n";
    for candidate in candidates {
        out += &format!("| {} | {} |  \n", candidate.0.to_string(), candidate.1);
    }

    out
}

fn table_winners(results: &ElectionResult<AccountId, Perbill>) -> String {
    let mut out = String::new();

    out += "  \n";
    out += "| Account | Approval stake |  \n";
    out += "| --- | --- |  \n";
    for winner in &results.winners {
        out += &format!("| {} | {} |  \n", winner.0.to_string(), winner.1);
    }

    out
}
