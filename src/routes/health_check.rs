use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn health_check(_req: HttpRequest) -> impl Responder {
    //this is a conflict
    HttpResponse::Ok().finish();
    //this will create a conflict
}
