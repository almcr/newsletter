use std::net::TcpListener;
use actix_web::{HttpServer, dev::Server, App};
use crate::routes::{subscribe, health_check};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
  let server = HttpServer::new(|| {
    App::new()
      .service(health_check)
      .service(subscribe)
  })
  .listen(listener)?
  .run();

  Ok(server)
}
