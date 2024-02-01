use actix_web::{get, web, HttpResponse, Scope};
use chrono::NaiveDate;

use crate::database::{db_mysql::DBClient, db_socket::DBSocket};

pub fn register(db_client: web::Data<DBClient>, scope: Scope) -> Scope {
    scope.service(from_date).app_data(db_client)
}

#[get("/get/{date}")]
async fn from_date(path: web::Path<NaiveDate>, db_client: web::Data<DBClient>) -> HttpResponse {
    let date = path.into_inner();

    let Ok(plans) = db_client.get_barplan_from_date(date) else {
        return HttpResponse::InternalServerError().finish();
    };
    
    HttpResponse::Ok().json(plans)
}
