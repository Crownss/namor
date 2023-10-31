use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::common::errors::Res;

impl super::SubChannelDataStore {
    pub fn new(the_client: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        Self { the_client }
    }
    pub async fn get_all(&self) -> Res<(Vec<String>, i64)> {
        let query = "select distinct(c.subchannel) from partnerchannel pc inner join channel c on c.id = pc.chid where c.trxtype='deposit' and c.subchannel!='';";
        let query_1 = "select count(distinct(c.subchannel)) from partnerchannel pc inner join channel c on c.id = pc.chid where c.trxtype='deposit' and c.subchannel!='';";
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
        let mut res = Vec::new();
        let total: i64 = do_query_1.get(0);
        for me in do_query {
            let name: String = me.get(0);
            res.push(name);
        }
        Ok((res, total))
    }
}
