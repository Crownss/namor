use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{types::FromSql, NoTls};

use crate::common::errors::Res;

use super::entity::ChannelListEntity;

impl super::ChannelDataStore {
    pub fn new(the_client: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        Self { the_client }
    }
    pub async fn get_all(&self) -> Res<(Vec<String>, i64)> {
        let query = "select distinct(c.chname) from partnerchannel pc inner join channel c on c.id = pc.chid where c.trxtype='deposit'";
        let query_1 = "select count(distinct(c.chname))  from partnerchannel pc inner join channel c on c.id = pc.chid where c.trxtype = 'deposit' and c.chname!='';";
        let do_query = self
            .the_client
            .get()
            .await
            .unwrap()
            .query(query, &[])
            .await?;
        let do_query_1 = self
            .the_client
            .get()
            .await
            .unwrap()
            .query_one(query_1, &[])
            .await?;
        let total: i64 = do_query_1.get(0);
        let mut res = Vec::new();
        for me in do_query {
            let name: String = me.get(0);
            res.push(name);
        }
        Ok((res, total))
    }
}
