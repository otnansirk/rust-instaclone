use std::fs;
use std::io::Read;

use actix_web::{HttpResponse};
use actix_web::web::{self, Json};
use serde::{Serialize, Deserialize};
#[derive(Deserialize, Serialize)]
struct Reels {
    caption: String,
    r#type: String
}
async fn list() -> String {
    format!("asdsadasd")
}

async fn create(req: Json<Reels>) -> HttpResponse {
    HttpResponse::Ok().json(req)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("reels")
            .route("", web::get().to(list))
            .route("", web::post().to(create))
    );
}