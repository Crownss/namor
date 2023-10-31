use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

use crate::common::errors::Res;

use super::entity::PartnerListEntity;

impl super::PartnerDataStore {
    pub fn new(the_client: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        Self { the_client }
    }
    pub async fn get_all(&self) -> Res<Vec<PartnerListEntity>> {
        let query = "select distinct(p.id), p.pname from partner p inner join trx t on t.pid = p.id where t.trxtype='deposit';";
        let do_query = self
            .the_client
            .get()
            .await
            .unwrap()
            .query(query, &[])
            .await?;
        let mut res = Vec::new();
        for me in do_query {
            let id: uuid::Uuid = me.get(0);
            let name: String = me.get(1);
            let temp = PartnerListEntity { id, name };
            res.push(temp)
        }
        Ok(res)
    }
}
