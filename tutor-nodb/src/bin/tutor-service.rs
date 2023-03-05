use actix_web::{web, App, HttpServer};

use tutor_nodb::{router, State};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Application state.
    let state = web::Data::new(State::default());

    // Creates a web app factory.
    let app = move || App::new().app_data(state.clone()).configure(router::init);

    // Starts a web server.
    HttpServer::new(app)
        .bind("127.0.0.1:3000")?
        .bind("[::1]:3001")?
        .run()
        .await
}
