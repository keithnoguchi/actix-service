use actix_web::HttpResponse;

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().body(
        r#"
            <!doctype html>
            <p>I'm healthy</p>
        "#,
    )
}
