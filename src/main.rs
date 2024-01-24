use actix_files::{self, NamedFile};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Error};
use log::debug;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    debug!("Starting Server");
    HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new(
                "/assets",
                "frontend/dist/assets",
            ))
            .service(api::register(web::scope("/api")))
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


