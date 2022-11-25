
mod routes;

use actix_web::{App, HttpServer, HttpResponse, web};
use actix_web::http::{StatusCode};
use std::io::Result;

use routes::base_router;

async fn not_found() -> HttpResponse {
    HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("Aplication/json")
        .body("Route Not Found")
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| 
        App::new()
        .configure(base_router::init_routes)
        .default_service(web::route().to(not_found))
    )
    .bind(("localhost", 8080))?
    .run()
    .await
}