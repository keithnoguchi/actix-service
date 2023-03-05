use actix_web::{web, App, HttpServer};

use tutor_nodb::{router, State};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initializes the global application state.
    let state = web::Data::new(State::new());

    // Creates a web app factory.
    let app = move || {
        App::new()
            .app_data(state.clone())
            .configure(router::default_routes)
            .configure(router::course_routes)
    };

    // Starts a web server.
    HttpServer::new(app)
        .bind("127.0.0.1:3000")?
        .bind("[::1]:3001")?
        .run()
        .await
}
