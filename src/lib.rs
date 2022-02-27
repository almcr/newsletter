use actix_web::{get, App, HttpResponse, HttpServer, Responder, dev::Server};

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("hello world!")
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
  HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
  let server = HttpServer::new(|| {
    App::new()
      .service(hello)
      .service(health_check)
  })
  .bind(("127.0.0.1", 8080))?
  .run();

  Ok(server)
}