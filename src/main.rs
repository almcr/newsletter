use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("hello world!")
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
  HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(hello)
      .service(health_check)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
