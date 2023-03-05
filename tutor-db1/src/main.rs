use tutor_db1::router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = || actix_web::App::new().configure(router::general);

    actix_web::HttpServer::new(app)
        .bind("127.0.0.1:3000")?
        .bind("[::1]:3000")?
        .run()
        .await?;

    Ok(())
}
