use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

pub mod data_store;
pub mod entity;
pub mod repository;

pub struct SubChannelDataStore {
    the_client: Pool<PostgresConnectionManager<NoTls>>,
}

pub struct SubChannelRepo {
    repo: SubChannelDataStore,
}
