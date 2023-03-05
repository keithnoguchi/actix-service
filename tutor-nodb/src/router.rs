use actix_web::web;

use crate::handler;

pub fn default_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(handler::health));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/courses")
                .route("/", web::post().to(handler::new_course))
                .route("/{tutor_id}", web::get().to(handler::get_courses)));
}
