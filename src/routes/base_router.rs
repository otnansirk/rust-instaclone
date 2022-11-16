use actix_web::web;

use super::{reels, post};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    reels::init_routes(cfg); 
    post::init_routes(cfg);
}