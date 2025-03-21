use super::*;

#[derive(Default, Debug)]
pub struct ElectionsDataOnChain {
    pub block_hash: <SubstrateConfig as Config>::Hash,
    pub desired_members: u32,
    pub desired_runners_up: u32,
    pub election_rounds: u32,
    pub members: Vec<SeatHolder<AccountId, u128>>,
    pub runners_up: Vec<SeatHolder<AccountId, u128>>,
    pub candidates: Vec<(AccountId, u128)>,
    pub voting: Vec<(AccountId, Voter<AccountId, u128>)>,
}

pub async fn download_onchain_elections_data(args: &Args) -> Result<ElectionsDataOnChain> {
    // Connect to node
    event!(Level::INFO, "Connecting to {}  ", &args.uri);
    let api = OnlineClient::<SubstrateConfig>::from_url(&args.uri).await?;

    // Prepare block hash to operate on
    let block_hash = match args.at {
        Some(hash) => {
            event!(Level::INFO, "Block hash: {:?}  ", hash);
            hash
        }
        None => {
            let latest_block_hash = api.blocks().at_latest().await?.hash();
            event!(Level::INFO, "Latest block hash: {:?}  ", latest_block_hash);
            latest_block_hash
        }
    };

    // Fetch constants: DesiredMembers, DesiredRunnersUp
    let desired_members = api
        .constants()
        .at(&substrate::constants().elections().desired_members())?;
    let desired_runners_up = api
        .constants()
        .at(&substrate::constants().elections().desired_runners_up())?;
    event!(
        Level::INFO,
        "DesiredMembers = {}, DesiredRunnersUp = {}",
        desired_members,
        desired_runners_up
    );

    // Fetch Elections data
    let storage = api.storage().at(block_hash);

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

    Ok(ElectionsDataOnChain {
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
