use crate::common::errors::Res;
pub mod model;
pub mod service;

#[async_trait::async_trait]
pub trait ISubChannelRepository {
    async fn get_all(&self) -> Res<(Vec<String>, i64)>;
}
