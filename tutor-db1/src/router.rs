use actix_web::web;

use crate::handler;

pub fn general(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(handler::index))
        .route("/health", web::get().to(handler::health));
}
