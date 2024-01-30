use actix_web::{get, web, HttpResponse, Scope};
use chrono::{DateTime, FixedOffset};

pub fn register(scope: Scope) -> Scope {
    scope.service(from_date)
}

#[get("/get/{date}")]
async fn from_date(path: web::Path<DateTime<FixedOffset>>) -> HttpResponse {
    HttpResponse::Ok().json("Test")
}
