use super::*;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone)]
pub struct OnchainDataProvider<C: Config> {
    api: OnlineClient<C>,
}

impl<C: Config> OnchainDataProvider<C> {
    pub async fn new(uri: &str) -> Result<Self> {
        let api = OnlineClient::<C>::from_url(uri).await?;

        Ok(Self { api })
    }
}

impl OnchainElectionsDataProvider for OnchainDataProvider<SubstrateConfig> {
    async fn elections_at_blockhash(&self, hash: Option<Hash>) -> Result<OnchainElectionsData> {
        // Prepare block hash to operate on
        let block_hash = match hash {
            Some(hash) => {
                event!(Level::DEBUG, "Block hash: {:?}  ", hash);
                hash
            }
            None => {
                let latest_block_hash = self.api.blocks().at_latest().await?.hash();
                event!(Level::DEBUG, "Latest block hash: {:?}  ", latest_block_hash);
                latest_block_hash
            }
        };

        // Fetch constants: DesiredMembers, DesiredRunnersUp
        let desired_members = self
            .api
            .constants()
            .at(&substrate::constants().elections().desired_members())?;
        let desired_runners_up = self
            .api
            .constants()
            .at(&substrate::constants().elections().desired_runners_up())?;

        // Fetch Elections data
        let storage = self.api.storage().at(block_hash);

        // Fetch number of election rounds
        let election_rounds = storage
            .fetch(&substrate::storage().elections().election_rounds())
            .await?
            .expect("ElectionRounds not found in storage");

        // Fetch Members
        let members = storage
            .fetch(&substrate::storage().elections().members())
            .await?
            .expect("Members not found in storage");

        // Fetch RunnersUp
        let runners_up = storage
            .fetch(&substrate::storage().elections().runners_up())
            .await?
            .expect("RunnersUp not found in storage");

        // Fetch Candidates
        let mut candidates = Vec::new();
        if let Some(mut c) = storage
            .fetch(&substrate::storage().elections().candidates())
            .await?
        {
            candidates.append(&mut c);
        }

        // Fetch Voting
        let mut voting = Vec::new();
        let mut voting_iter = storage
            .iter(substrate::storage().elections().voting_iter())
            .await?;
        while let Some(Ok(kv)) = voting_iter.next().await {
            let voter = kv.value;
            let who: [u8; 32] = kv.key_bytes[40..].try_into().unwrap();
            let voter_account = <SubstrateConfig as subxt::Config>::AccountId::from(who);
            voting.push((voter_account, voter));
        }

        Ok(OnchainElectionsData {
            block_hash,
            desired_members,
            desired_runners_up,
            election_rounds,
            members,
            runners_up,
            candidates,
            voting,
        })
    }
}

impl OnchainIdentityProvider for OnchainDataProvider<SubstrateConfig> {
    async fn map_elections_identities(&self, elections: &mut ApiElectionResults) -> Result<()> {
        use substrate::runtime_types::pallet_identity::types::Data;

        // TODO: This would really benefit from cache shared among all workers

        // ApiAccount's with resolved names
        let mut resolved: HashMap<String, String> = HashMap::new();

        // Resolve candidates
        for candidate in &elections.election_data.final_results {
            let account = subxt::utils::AccountId32::from_str(&candidate.id.address)?;
            let storage = substrate::storage().identity().identity_of(&account);

            let resp = self
                .api
                .storage()
                .at_latest()
                .await?
                .fetch(&storage)
                .await?;

            if let Some(identity) = resp {
                let display_name = match identity.info.display {
                    Data::Raw1(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw2(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw3(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw4(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw5(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw6(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw7(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw8(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw9(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw10(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw11(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw12(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw13(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw14(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw15(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw16(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw17(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw18(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw19(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw20(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw21(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw22(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw23(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw24(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw25(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw26(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw27(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw28(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw29(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw30(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw31(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    Data::Raw32(raw_vec) => String::from_utf8(raw_vec.to_vec())?,
                    _ => {
                        event!(
                            Level::WARN,
                            "display name not recognized for address {}: {:?}",
                            candidate.id.address,
                            identity.info.display
                        );
                        continue;
                    }
                };

                resolved.insert(candidate.id.address.clone(), display_name);
            }
        }

        // Map candidate names in final_results
        for candidate in elections.election_data.final_results.iter_mut() {
            if let Some(display_name) = resolved.get(&candidate.id.address) {
                candidate.id.display_name = Some(display_name.clone());
            }
        }

        // Map candidate names in candidates
        for candidate in elections.election_data.candidates.iter_mut() {
            if let Some(display_name) = resolved.get(&candidate.id.address) {
                candidate.id.display_name = Some(display_name.clone());
            }
        }

        // Map candidate names in rounds.scores
        for round in elections.election_data.rounds.iter_mut() {
            for candidate in round.scores.iter_mut() {
                if let Some(display_name) = resolved.get(&candidate.id.address) {
                    candidate.id.display_name = Some(display_name.clone());
                }
            }
        }

        Ok(())
    }
}
