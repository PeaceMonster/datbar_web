use chrono::NaiveDate;
use mysql::{prelude::Queryable, Pool};

use super::db_socket::{Barplan, Bartender, DBSocket};

#[derive(Clone)]
pub struct DBClient {
    pool: Pool,
}

impl DBSocket for DBClient {
    fn get_barplan(&self, date: NaiveDate) -> anyhow::Result<Barplan> {
        todo!()
    }

    fn get_bartenders(&self) -> Result<Vec<Bartender>, anyhow::Error> {
        let mut conn = self.pool.get_conn().unwrap();

        let result = conn
            .query_map(
                r"SELECT name,username,aktiv FROM Bartendere",
                |(name, username, aktiv)| Bartender {
                    name,
                    username,
                    active: aktiv,
                },
            )?;

        Ok(result)
    }
}

impl DBClient {
    pub async fn new() -> Result<DBClient, anyhow::Error> {
        Ok(DBClient {
            pool: Pool::new("mysql://server@127.0.0.1/datbarweb")?,
        })
    }
}
