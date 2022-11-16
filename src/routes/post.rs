use actix_web::{post, HttpResponse, web};
use serde::Serialize;

#[derive(Serialize)]
struct CreatePost {
    source: String,
    caption: String
}

#[post("")]
async fn create_post() -> HttpResponse {
    let data = CreatePost {
        source: "Source url".to_string(),
        caption: "This my first post".to_string()
    };

    HttpResponse::Ok().json(data)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("posts")
            .service(create_post)
    );
}