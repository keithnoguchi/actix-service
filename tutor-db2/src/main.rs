use tutor_db2::router;

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    let app = || actix_web::App::new().configure(router::generic);

    actix_web::HttpServer::new(app)
        .bind("127.0.0.1:3000")?
        .bind("[::1]:3000")?
        .run()
        .await?;

    Ok(())
}
