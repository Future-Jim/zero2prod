use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn health_check(_req: HttpRequest) -> impl Responder {
    //this is a csadasdonflictdsfsd
    //dsfsdfsd
    HttpResponse::Ok().finish();
    //sdfsdfsdfsthis will create a conflict
}
