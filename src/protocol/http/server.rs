use actix_web::web::{get, scope, Data};
use actix_web::{guard, middleware::Logger, App, HttpServer};
use std::sync::Arc;
use std::time::Duration;

use crate::data::repo::{channel as channelRepo, subchannel as subchannelRepo};
use crate::interactor::{channel, subchannel};

pub struct Services {
    pub channel_service: Arc<channel::service::ChannelService<channelRepo::ChannelRepo>>,
    pub subchannel_service:
        Arc<subchannel::service::SubChannelService<subchannelRepo::SubChannelRepo>>,
}

pub struct Server {
    pub graceful: u8,
    pub client_timeout: u8,
    pub port: u16,
    pub service: Services,
}

impl Server {
    pub fn new(graceful: u8, client_timeout: u8, port: u16, service: Services) -> Self {
        Self {
            graceful,
            client_timeout,
            port,
            service,
        }
    }

    pub async fn run(&self) {
        let channel_service = Data::new(self.service.channel_service.clone());
        let subchannel_serivce = Data::new(self.service.subchannel_service.clone());
        HttpServer::new(move|| {
            App::new()
                .wrap(super::middleware::Logging)
                .wrap(Logger::new(
            "\n uri: %U\n got IP: %{r}a\n with sec: %T\n in: %r\n and return: %s\n with user-agent: %{User-Agent}i",
        ))
        .service(
            scope("/v1/api")
                .guard(
                    guard::Any(guard::Get())
                    // .or(guard::Post()))
                )
                .app_data(channel_service.clone())
                .route("/channel", get().to(super::handler::channel::get_all))
                .app_data(subchannel_serivce.clone())
                .route("/subchannel", get().to(super::handler::subchannel::get_all))
        )})
        .bind(format!("0.0.0.0:{}", self.port))
        .unwrap()
        .shutdown_timeout(self.graceful as u64)
        .client_request_timeout(Duration::new(self.client_timeout as u64, 0))
        // .workers(0)
        .run()
        .await
        .unwrap();
    }
}
