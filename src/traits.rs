use super::*;

pub trait OnchainElectionsDataProvider {
    async fn elections_at_blockhash(&self, hash: Option<Hash>) -> Result<OnchainElectionsData>;
}
