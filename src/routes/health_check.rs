use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn health_check(_req: HttpRequest) -> impl Responder {
    println!("heldfsdfdsfdso");
    println!("hello");
}
