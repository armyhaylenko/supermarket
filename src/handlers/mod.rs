mod test_endpoints;

use crate::config::crypto::CryptoService;
use crate::db::{Action, SupermarketRepository, UserRepository};
use crate::models::*;
use crate::utils::{handle_query_and_auth, has_auth_level};
use actix_web::{
    get, post, web,
    web::{Form, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use futures::TryFutureExt;
use std::sync::Arc;
use validator::Validate;

#[get("/healthcheck")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("Working!")
}
#[post("/create_user")]
pub async fn create_new_user(
    req: HttpRequest,
    form: Form<NewUser>,
    user_repo: web::Data<Arc<UserRepository>>,
    crypto_svc: web::Data<Arc<CryptoService>>,
) -> impl Responder {
    if !has_auth_level(&req, "manager".to_owned()) {
        HttpResponse::Unauthorized()
            .body("The authorization header is not present or is malformed, or the user does not have access to this resource.")
    } else {
        let new_user_unvalidated = form.0;

        match new_user_unvalidated.validate() {
            Ok(_) => match user_repo.create_user(new_user_unvalidated, &*crypto_svc).await {
                Ok(usr) => HttpResponse::Ok().body(format!("User {} was successfully added", usr.username)),
                Err(e) => HttpResponse::UnprocessableEntity().body(format!("{}", e)),
            },
            Err(e) => HttpResponse::UnprocessableEntity().body(format!("{}", e)),
        }
    }
}

#[get("/user/{username}")]
pub async fn get_user(req: HttpRequest, user_repo: web::Data<Arc<UserRepository>>) -> impl Responder {
    let username = req.match_info().get("username").unwrap();

    if !has_auth_level(&req, "manager".to_owned()) {
        HttpResponse::Unauthorized()
            .body("The authorization header is not present or is malformed, or the user does not have access to this resource.")
    } else {
        match user_repo.get_user_by_username(&username).await {
            Ok(maybe_user) => match maybe_user {
                Some(usr) => HttpResponse::Ok().body(format!(
                    "{}",
                    serde_json::to_string(&usr)
                        .unwrap_or_else(|_| "The user was malformed in the database, or the query didn't work correctly".to_owned())
                )),
                None => HttpResponse::NotFound().body(format!("Could not find user {}", &username)),
            },
            Err(e) => HttpResponse::InternalServerError().body(format!("The database was not able to process get_user request: {}", e)),
        }
    }
}

#[post("/employee")] // ?action={create, delete, update}
pub async fn employee(req: HttpRequest, body: web::Json<ShopEmployee>, shop_repo: web::Data<Arc<SupermarketRepository>>) -> impl Responder {
    let emp = body.into_inner();
    let action_fut = Action::from_req(&req, emp);
    handle_query_and_auth(&req, action_fut.and_then(|act| shop_repo.handle_employee(act)).await, "manager")
}

#[post("/client_card")]
pub async fn client_card(
    req: HttpRequest,
    body: web::Json<ClientCard>,
    shop_repo: web::Data<Arc<SupermarketRepository>>,
) -> impl Responder {
    let cc = body.into_inner();
    let action_fut = Action::from_req(&req, cc);
    handle_query_and_auth(&req, action_fut.and_then(|act| shop_repo.handle_client_card(act)).await, "manager")
}

#[post("/manufacturer")]
pub async fn manufacturer(
    req: HttpRequest,
    body: web::Json<Manufacturer>,
    shop_repo: web::Data<Arc<SupermarketRepository>>,
) -> impl Responder {
    let cc = body.into_inner();
    let action_fut = Action::from_req(&req, cc);
    handle_query_and_auth(&req, action_fut.and_then(|act| shop_repo.handle_manufacturer(act)).await, "manager")
}

#[post("/product")]
pub async fn product(req: HttpRequest, body: web::Json<Product>, shop_repo: web::Data<Arc<SupermarketRepository>>) -> impl Responder {
    let p = body.into_inner();
    let action_fut = Action::from_req(&req, p);
    handle_query_and_auth(&req, action_fut.and_then(|act| shop_repo.handle_product(act)).await, "manager")
}

#[post("/owned_product")]
pub async fn owned_product(
    req: HttpRequest,
    body: web::Json<OwnedProduct>,
    shop_repo: web::Data<Arc<SupermarketRepository>>,
) -> impl Responder {
    let op = body.into_inner();
    let action_fut = Action::from_req(&req, op);
    handle_query_and_auth(
        &req,
        action_fut.and_then(|act| shop_repo.handle_owned_product(act)).await,
        "manager",
    )
}

#[post("/category")]
pub async fn category(req: HttpRequest, body: web::Json<Category>, shop_repo: web::Data<Arc<SupermarketRepository>>) -> impl Responder {
    let c = body.into_inner();
    let action_fut = Action::from_req(&req, c);
    handle_query_and_auth(&req, action_fut.and_then(|act| shop_repo.handle_category(act)).await, "manager")
}

#[post("/waybill")]
pub async fn waybill(req: HttpRequest, body: web::Json<Waybill>, shop_repo: web::Data<Arc<SupermarketRepository>>) -> impl Responder {
    let w = body.into_inner();
    let action_fut = Action::from_req(&req, w);
    handle_query_and_auth(&req, action_fut.and_then(|act| shop_repo.handle_waybill(act)).await, "manager")
}

#[post("/return_agreement")]
pub async fn return_agreement(
    req: HttpRequest,
    body: web::Json<ReturnAgreement>,
    shop_repo: web::Data<Arc<SupermarketRepository>>,
) -> impl Responder {
    let ra = body.into_inner();
    let action_fut = Action::from_req(&req, ra);
    handle_query_and_auth(
        &req,
        action_fut.and_then(|act| shop_repo.handle_return_agreement(act)).await,
        "manager",
    )
}

#[post("/create_receipt")]
pub async fn create_receipt(
    req: HttpRequest,
    body: web::Json<CreateReceipt>,
    shop_repo: web::Data<Arc<SupermarketRepository>>,
) -> impl Responder {
    handle_query_and_auth(&req, shop_repo.handle_create_receipt(body.into_inner()).await, "manager")
}

#[post("/delete_receipt")]
pub async fn delete_receipt(
    req: HttpRequest,
    body: web::Json<Receipt>,
    shop_repo: web::Data<Arc<SupermarketRepository>>,
) -> impl Responder {
    handle_query_and_auth(&req, shop_repo.handle_delete_receipt(body.into_inner()).await, "manager")
}

#[post("/update_sale")]
pub async fn update_sale(req: HttpRequest, body: web::Json<Sale>, shop_repo: web::Data<Arc<SupermarketRepository>>) -> impl Responder {
    handle_query_and_auth(&req, shop_repo.handle_update_sale(body.into_inner()).await, "cashier")
}

pub fn init_app_config(server_cfg: &mut ServiceConfig) -> () {
    let tests_scope = web::scope("/tests").service(test_endpoints::get_most_recent_employee);
    let api_scope = web::scope("/api")
        .service(tests_scope)
        .service(employee)
        .service(client_card)
        .service(manufacturer)
        .service(product)
        .service(owned_product)
        .service(category)
        .service(waybill)
        .service(return_agreement)
        .service(create_receipt)
        .service(delete_receipt)
        .service(update_sale);
    let admin_scope = web::scope("/admin").service(create_new_user).service(get_user);
    server_cfg.service(healthcheck).service(api_scope).service(admin_scope);
}
