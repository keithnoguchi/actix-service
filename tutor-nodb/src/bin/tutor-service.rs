use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use tutor_nodb::State;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Application state.
    let state = web::Data::new(State::default());

    // Creates a web app factory.
    let app = move || App::new().app_data(state.clone()).configure(routes);

    // Starts a web server.
    HttpServer::new(app)
        .bind("127.0.0.1:3000")?
        .bind("[::1]:3001")?
        .run()
        .await
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}

async fn health(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json("Hello.  It's up and running")
}
