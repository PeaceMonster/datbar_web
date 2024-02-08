use chrono::{DateTime, FixedOffset, NaiveDate};
use mysql::{params, prelude::Queryable, Pool};

use super::db_socket::{Barplan, Bartender, DBSocket};

#[derive(Clone)]
pub struct DBClient {
    pool: Pool,
}

impl DBSocket for DBClient {
    fn get_barplan_from_date(&self, date: NaiveDate) -> anyhow::Result<Vec<Barplan>> {
        let mut conn = self.pool.get_conn().unwrap();
        
        let vagter : Vec<(NaiveDate, i64, Bartender)> = conn.query_map(format!(r"SELECT Barvagter.dato,Barvagter.vagtId,Bartendere.* from Barvagter INNER JOIN Bartendere on Bartendere.bartenderId = Barvagter.ansvarlig WHERE dato >= '{}'", date),
        |(date, vagt_id, _, name, username, active) : (String, i64, i64, String, String, bool)|  {
            (NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap(), vagt_id, Bartender {
                name,
                username,
                active,
            })
        })?;


        println!("{:?}", vagter);

        let result : Vec<Barplan> = vagter.into_iter().map(|(date, vagt_id, responisble)| {
            let bartenders = conn.query_map(format!("SELECT Bartendere.* from VagtTilBartender INNER JOIN Bartendere on Bartendere.bartenderId = VagtTilBartender.bartenderId WHERE VagtTilBartender.vagtId = {};", vagt_id),
        |(_, name, username, active): (i64,String,String,bool)| Bartender {name, username, active}).unwrap();
            Barplan {
                date,
                responisble,
                bartenders,
            }
        }).collect();

        Ok(result)
    }

    fn get_bartenders(&self) -> Result<Vec<Bartender>, anyhow::Error> {
        let mut conn = self.pool.get_conn().unwrap();

        let result = conn.query_map(
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
