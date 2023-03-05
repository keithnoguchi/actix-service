use crate::State;
use actix_web::{web, HttpResponse};

pub async fn health(state: web::Data<State>) -> HttpResponse {
    let msg = &state.message;
    let mut count = state.count.lock().unwrap();
    let resp = format!("{msg} {count} times");
    *count += 1;
    HttpResponse::Ok().json(resp)
}
