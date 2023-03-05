use std::sync::atomic::Ordering;

use crate::db;
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
    state: web::Data<State>,
    new_course: web::Json<Course>,
) -> HttpResponse {
    match db::add_course(&state.db, new_course.into()).await {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub(crate) async fn get_courses(state: web::Data<State>, path: web::Path<u32>) -> HttpResponse {
    let tutor_id = path.into_inner();
    match db::get_courses(&state.db, tutor_id).await {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub(crate) async fn get_course(
    state: web::Data<State>,
    path: web::Path<(u32, u32)>,
) -> HttpResponse {
    let (tutor_id, course_id) = path.into_inner();
    match db::get_course(&state.db, tutor_id, course_id).await {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
