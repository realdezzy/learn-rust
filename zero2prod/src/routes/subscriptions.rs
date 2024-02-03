use actix_web::{ web, App, HttpRequest, HttpServer, HttpResponse};
use actix_web::dev::Server;
use std::net::TcpListener;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}