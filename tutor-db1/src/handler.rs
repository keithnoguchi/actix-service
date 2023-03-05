use crate::model::Course;

use actix_web::{web, HttpResponse};

pub(crate) async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

pub(crate) async fn health() -> HttpResponse {
    HttpResponse::Ok().body(
        r#"
            <!doctype html>
            <p>I'm healthy</p>
        "#,
    )
}

pub(crate) async fn new_course(_new_course: web::Json<Course>) -> HttpResponse {
    HttpResponse::Ok().json("success")
}
