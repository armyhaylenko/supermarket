use crate::db::SupermarketRepository;
use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Arc;

#[get("/get_most_recent_employee")]
pub async fn get_most_recent_employee(shop_repo: web::Data<Arc<SupermarketRepository>>) -> impl Responder {
    match shop_repo.get_most_recent_employee().await {
        Ok(empl_id) => HttpResponse::Ok().body(format!("{}", serde_json::to_string(&empl_id).unwrap())),
        Err(e) => HttpResponse::InternalServerError().body(format!("{:?}", e)),
    }
}
