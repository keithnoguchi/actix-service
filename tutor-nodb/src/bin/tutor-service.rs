use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Creates a web app factory.
    let app = || App::new().configure(routes);

    // Starts a web server.
    HttpServer::new(app)
        .bind("127.0.0.1:3000")?
        .bind("[::1]:3001")?
        .run()
        .await
}

fn routes(_cfg: &mut web::ServiceConfig) {}
