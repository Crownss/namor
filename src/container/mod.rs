use std::sync::Arc;

use tracing::info;

use crate::{
    configuration::get_configurations,
    data::{
        infra::psql::{check_connection, get_connection},
        repo::{
            channel::{ChannelDataStore, ChannelRepo},
            subchannel::{SubChannelDataStore, SubChannelRepo},
        },
    },
    interactor::{channel::service::ChannelService, subchannel::service::SubChannelService},
    protocol::http::server::{Server, Services},
};

pub async fn start() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    let config = get_configurations();
    info!("starting http service with port: {}", config.server.port);
    let get_psql = get_connection().await.unwrap();
    let _ = check_connection().await;

    //channel//
    let channel_data_store = ChannelDataStore::new(get_psql.clone());
    let channel_repo = ChannelRepo::new(channel_data_store);
    let channel_service = Arc::new(ChannelService::new(channel_repo));
    //channel//

    //subchannel//
    let subchannel_data_store = SubChannelDataStore::new(get_psql.clone());
    let subchannel_repo = SubChannelRepo::new(subchannel_data_store);
    let subchannel_service = Arc::new(SubChannelService::new(subchannel_repo));
    //subchannel//

    let server = Server::new(
        config.server.graceful,
        config.server.client_timeout,
        config.server.port,
        Services {
            channel_service,
            subchannel_service,
        },
    );
    server.run().await;
    Ok(())
}
