use actix_web::web;
use sqlx::postgres::PgPool;
use tutor_db1::{router, state};

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv()?;

    // Prepares the application state.
    let db_url = std::env::var("DATABASE_URL")?;
    let db_pool = PgPool::connect(&db_url).await?;
    let state = web::Data::new(state::State::new(db_pool));

    // Creates a app factory.
    let app = move || {
        actix_web::App::new()
            .app_data(state.clone())
            .configure(router::general)
            .configure(router::courses)
    };

    // Starts the server.
    actix_web::HttpServer::new(app)
        .bind("127.0.0.1:3000")?
        .bind("[::1]:3000")?
        .run()
        .await?;

    Ok(())
}
