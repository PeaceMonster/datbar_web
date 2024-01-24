use actix_web::{web, HttpResponse, Responder, Scope};



pub fn register(scope: Scope) -> Scope {
    scope
        .route("/hello", web::get().to(hello))
} 



async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

    