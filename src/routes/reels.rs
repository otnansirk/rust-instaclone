use actix_web::{get, HttpResponse, web};
use serde::Serialize;


#[derive(Serialize)]
struct ReelsList {
    caption: String
}

#[get("")]
async fn get_list() -> HttpResponse {
    let data: Vec<ReelsList> = vec![ReelsList{
        caption: "Reels caption".to_string()
    }];

    HttpResponse::Ok().json(data)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("reels")
            .service(get_list)
    );
}