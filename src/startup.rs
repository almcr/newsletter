use std::net::TcpListener;
use actix_web::{HttpServer, dev::Server, App, web};
use actix_web::middleware::Logger;
use sqlx::PgPool;
use crate::routes::{subscribe, health_check};

pub fn run(
  listener: TcpListener,
  connection: PgPool
) -> Result<Server, std::io::Error> {
  
  // create arc of the connection 
  let pgpool = web::Data::new(connection);

  let server = HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      .service(health_check)
      .service(subscribe)
      .app_data(pgpool.clone())
  })
  .listen(listener)?
  .run();

  Ok(server)
}
