use actix_web::web;

use crate::handler;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(handler::health));
}
