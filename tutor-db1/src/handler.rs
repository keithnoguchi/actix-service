use std::sync::atomic::Ordering;

use crate::model::Course;
use crate::state::State;

use actix_web::{web, HttpResponse};

pub(crate) async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

pub(crate) async fn health(state: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().body(format!(
        r#"
            <!doctype html>
            <p>{} visited {} times</p>
        "#,
        state.health_check_string,
        state.visit_count.fetch_add(1, Ordering::Relaxed),
    ))
}

pub(crate) async fn new_course(
    _state: web::Data<State>,
    _new_course: web::Json<Course>,
) -> HttpResponse {
    HttpResponse::NotImplemented().json("not implemented")
}

pub(crate) async fn get_courses(_state: web::Data<State>, _path: web::Path<u32>) -> HttpResponse {
    HttpResponse::NotImplemented().json("not implemented")
}

pub(crate) async fn get_course(
    _state: web::Data<State>,
    _path: web::Path<(u32, u32)>,
) -> HttpResponse {
    HttpResponse::NotImplemented().json("not implemented")
}
