use chrono::NaiveDate;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Bartender {
    pub name: String,
    pub username: String,
    pub active: bool,
}

#[derive(Serialize, Debug)]
pub struct Barplan {
    pub date: NaiveDate,
    pub responisble: Bartender,
    pub bartenders: Vec<Bartender>,
}

pub trait DBSocket {
    fn get_barplan_from_date(&self, date: NaiveDate) -> anyhow::Result<Vec<Barplan>>;

    fn get_bartenders(&self) -> anyhow::Result<Vec<Bartender>>;
}
