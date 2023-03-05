use actix_web::web;

use crate::handler;

pub fn general(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(handler::index))
        .route("/health", web::get().to(handler::health));
}

pub fn courses(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(handler::new_course))
            .route("/{tutor_id}", web::get().to(handler::get_courses))
            .route(
                "/{tutor_id}/{course_id}",
                web::get().to(handler::get_course),
            ),
    );
}
