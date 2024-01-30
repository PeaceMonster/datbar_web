use chrono::{DateTime, FixedOffset};

use serde::Serialize;

#[derive(Serialize)]
pub struct Bartender {
    pub name: String,
    pub username: String,
    pub active: bool,
}

pub trait DBSocket {
    fn get_barplan(&self, date: DateTime<FixedOffset>);

    fn get_bartenders(&self) -> Result<Vec<Bartender>, anyhow::Error>;
}
