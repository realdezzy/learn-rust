use actix_web::{ web, App, HttpRequest, HttpServer, HttpResponse};



pub async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}