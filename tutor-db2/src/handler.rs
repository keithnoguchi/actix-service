use std::sync::atomic::Ordering;

use actix_web::{web, HttpResponse};

use crate::state::State;

pub(crate) async fn index(state: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().body(format!(
        r#"
            <!doctype html>
            <h1>Hi {}, you're {}</h1>
        "#,
        state.msg,
        state.counter.fetch_add(1, Ordering::Relaxed),
    ))
}
