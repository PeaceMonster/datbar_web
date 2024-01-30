use actix_web::{get, web::{self, get}, HttpResponse, Responder, Scope};

use crate::database::{db_mysql::DBClient, db_socket::DBSocket};

mod barplan;

pub fn register(db_client: web::Data<DBClient>, scope: Scope) -> Scope {
    scope
        .route("/hello", web::get().to(hello))
        .service(barplan::register(web::scope("/barplan")))
        .service(get_bartenders)
        .app_data(db_client)
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[get("/bartenders")]
async fn get_bartenders(db_client : web::Data<DBClient>) -> HttpResponse {
    let db_client = db_client.into_inner();
    let bartenders = match db_client.get_bartenders() {
        Err(e) => {
            log::error!("{}",e);
            return  HttpResponse::InternalServerError().finish()},
        Ok(r) => r
    };
    
    HttpResponse::Ok().json(bartenders)
}
