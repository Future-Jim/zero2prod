use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn health_check(_req: HttpRequest) -> impl Responder {
    //thdis is a conflict
    HttpResponse::Ok().finish();
    //thasdasis wdsadaill dsfsdfsd sdfcreate a conflict
    //sdsa
}
