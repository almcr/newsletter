use actix_web::{HttpResponse, post, web};

#[derive(serde::Deserialize)]
struct FormData {
  email: String,
  name: String
}


#[post("/subscriptions")]
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
  HttpResponse::Ok().finish()
}


