use super::*;

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
