use tracing::error;

use crate::{
    common::{errors::Res, responses::DefaultResponse},
    interactor::subchannel::{model::Response, ISubChannelRepository},
};
impl super::SubChannelRepo {
    pub fn new(repo: super::SubChannelDataStore) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl ISubChannelRepository for super::SubChannelRepo {
    async fn get_all(&self) -> Res<(Vec<String>, i64)> {
        self.repo.get_all().await
    }
}
