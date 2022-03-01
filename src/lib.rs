use std::net::TcpListener;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, dev::Server, post};

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("hello world!")
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
  HttpResponse::Ok().finish()
}

#[post("/subscriptions")]
async fn subscribe() -> HttpResponse {
  HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
  let server = HttpServer::new(|| {
    App::new()
      .service(hello)
      .service(health_check)
      .service(subscribe)
  })
  .listen(listener)?
  .run();

  Ok(server)
}