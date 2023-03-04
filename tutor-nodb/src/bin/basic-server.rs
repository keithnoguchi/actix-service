use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app = || App::new().configure(router);

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}

fn router(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}

async fn health() -> impl Responder {
    HttpResponse::Ok().json("Hello.  It's up and running")
}
