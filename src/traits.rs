use super::*;

pub trait OnchainElectionsDataProvider {
    async fn elections_at_blockhash(&self, hash: Option<Hash>) -> Result<OnchainElectionsData>;
}

pub trait OnchainIdentityProvider {
    async fn map_elections_identities(&self, elections: &mut ApiElectionResults) -> Result<()>;
}
