use actix_web::{web, HttpResponse};

use crate::State;

pub(crate) async fn health(state: web::Data<State>) -> HttpResponse {
    let msg = &state.message;
    let mut count = state.count.lock().unwrap();
    let resp = format!("{msg} {count} times");
    *count += 1;
    HttpResponse::Ok().json(resp)
}
