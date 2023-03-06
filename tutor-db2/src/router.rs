use actix_web::web;

use crate::handler::index;

pub fn generic(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index));
}
