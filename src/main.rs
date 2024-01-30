use actix_files::{self, NamedFile};
use actix_web::{get, web, App, Error, HttpServer};
use log::debug;

use database::db_mysql::DBClient;

mod api;
mod database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let db_client = DBClient::new().await.unwrap();
    

    debug!("Starting Server");
    HttpServer::new(move || {
        App::new()
            .service(actix_files::Files::new("/assets", "frontend/dist/assets"))
            .service(api::register(web::Data::new(db_client.clone()) ,web::scope("/api")))
            .service(admin_page)
            .service(frontpage)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/{page:.*}")]
async fn frontpage() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("frontend/dist/index.html")?)
}

#[get("/admin/{page:.*}")]
async fn admin_page() -> Result<NamedFile, Error> {
    Ok(NamedFile::open("frontend/dist/admin.html")?)
}
