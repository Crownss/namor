use super::model::Response;
use crate::common::responses::DefaultResponse;
use tracing::error;

pub struct ChannelService<R>
where
    R: super::IChannelRepository,
{
    repo: R,
}

impl<R> ChannelService<R>
where
    R: super::IChannelRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
    pub async fn get_all(&self) -> DefaultResponse<Response> {
        let getrepo = self.repo.get_all().await;
        let mut res = DefaultResponse {
            status: "001".to_string(),
            message: "OK".to_string(),
            data: None,
        };
        let mut resp = Response {
            total: 0,
            items: None,
        };
        match getrepo {
            Ok(r) => {
                resp.items = Some(r.0);
                resp.total = r.1;
                res.data = Some(resp)
            }
            Err(err) => {
                res.status = "5000".to_string();
                res.message = "something went wrong".to_string();
                error!("cannot get data from repository and got error: {err}")
            }
        }
        res
    }
}
