use actix_web::{web, HttpResponse};

use crate::State;

pub(crate) async fn health(state: web::Data<State>) -> HttpResponse {
    let msg = &state.message;
    let resp = {
        let mut count = state.count.lock().unwrap();
        let course_count = state.courses.read().unwrap().len();
        let resp = format!("{msg} {count} times with {course_count} courses");
        *count += 1;
        resp
    };
    HttpResponse::Ok().json(resp)
}
