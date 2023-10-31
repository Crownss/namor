use tracing::error;

use crate::{
    common::{responses::DefaultResponse, errors::Res},
    interactor::channel::{model::Response, IChannelRepository},
};
impl super::ChannelRepo {
    pub fn new(repo: super::ChannelDataStore) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl IChannelRepository for super::ChannelRepo {
    async fn get_all(&self) -> Res<(Vec<String>, i64)> {
        self.repo.get_all().await
    }
}
