use chrono::NaiveDate;

use serde::Serialize;

#[derive(Serialize)]
pub struct Bartender {
    pub name: String,
    pub username: String,
    pub active: bool,
}

pub struct Barplan {
    pub date: NaiveDate,
    pub responisble: Bartender,
    pub bartenders: Vec<Bartender>,
}

pub trait DBSocket {
    fn get_barplan(&self, date: NaiveDate) -> anyhow::Result<Barplan>;

    fn get_bartenders(&self) -> anyhow::Result<Vec<Bartender>>;
}
