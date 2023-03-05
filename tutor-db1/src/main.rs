use actix_web::{web, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = || actix_web::App::new().route("/", web::get().to(index));

    actix_web::HttpServer::new(app)
        .bind("127.0.0.1:3000")?
        .bind("[::1]:3000")?
        .run()
        .await?;

    Ok(())
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello World!")
}
