use tutor_db2::{router, state};
use sqlx::postgres::PgPool;

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let db = PgPool::connect(&db_url).await?;

    let state = actix_web::web::Data::new(state::State::new("Yo", db));
    let app = move || {
        actix_web::App::new()
            .app_data(state.clone())
            .configure(router::generic)
    };

    actix_web::HttpServer::new(app)
        .bind("127.0.0.1:3000")?
        .bind("[::1]:3000")?
        .run()
        .await?;

    Ok(())
}
