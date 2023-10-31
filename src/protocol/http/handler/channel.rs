use std::sync::Arc;

use actix_web::{http::header::ContentType, web::Data, HttpResponse};

use crate::{
    common::errors::Res, data::repo::channel::ChannelRepo, interactor::channel::service::ChannelService,
};

pub async fn get_all(service: Data<Arc<ChannelService<ChannelRepo>>>) -> Res<HttpResponse> {
    let getall = service.get_all().await;
    Ok(HttpResponse::Ok()
        .insert_header(ContentType::json())
        .json(getall))
}
