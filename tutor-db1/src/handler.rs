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

pub(crate) async fn new_course(_new_course: web::Json<Course>) -> HttpResponse {
    HttpResponse::Ok().json("success")
}
