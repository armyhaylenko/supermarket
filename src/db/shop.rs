use actix_web::{get, HttpResponse, Responder};

pub async fn get_manufacturer() -> impl Responder {
    HttpResponse::Ok().finish()
}
