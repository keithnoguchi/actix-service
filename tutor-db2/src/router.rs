use std::sync::atomic::Ordering;

use actix_web::{web, HttpResponse};

use crate::state::State;

pub fn generic(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index));
}

async fn index(state: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().body(format!(
        r#"
            <!doctype html>
            <h1>Hello {} visited {} times</h1>
        "#,
        state.msg,
        state.counter.fetch_add(1, Ordering::Relaxed),
    ))
}
