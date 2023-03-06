use actix_web::{web, HttpResponse};

pub fn generic(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index));
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body(
        r#"
            <!doctype html>
            <h1>Hello from router</h1>
        "#,
    )
}
