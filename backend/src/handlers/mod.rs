mod test_endpoints;

use crate::config::crypto::CryptoService;
use crate::db::{Action, SupermarketRepository, UserRepository};
use crate::models::{auth::*, *};
use crate::utils;
use actix_web::{
    get, post, web,
    web::{Form, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use futures::TryFutureExt;
use std::sync::Arc;
use tracing::debug;
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
    if !utils::has_auth_level(&req, "manager".to_owned()) {
        HttpResponse::Unauthorized()
            .body("The authorization header is not present or is malformed, or the user does not have access to this resource.")
    } else {
        let new_user_unvalidated = form.0;

        match new_user_unvalidated.validate() {
            Ok(_) => match user_repo.create_user(new_user_unvalidated, &*crypto_svc).await {
                Ok(usr) => {
                    let resp = HttpResponse::Ok()
                        .body("Success");

                    resp
                },
                Err(e) => HttpResponse::UnprocessableEntity().body(format!("{}", e)),
            },
            Err(e) => HttpResponse::UnprocessableEntity().body(format!("Invalid user data: {}", e)),
        }
    }
}

#[get("/user/{username}")]
pub async fn get_user(req: HttpRequest, user_repo: web::Data<Arc<UserRepository>>) -> impl Responder {
    let username = req.match_info().get("username").unwrap();

    if !utils::has_auth_level(&req, "manager".to_owned()) {
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

#[post("/login")]
pub async fn login(
    usr: Form<LoginUser>,
    user_repo: web::Data<Arc<UserRepository>>,
    crypto_service: web::Data<Arc<CryptoService>>,
) -> impl Responder {
    let body = usr.into_inner();
    match user_repo.login(&body, &**crypto_service).await {
        Ok(usr) => match utils::encode_user_to_token(&usr, std::env::var("JWT_DECODING_KEY").unwrap().as_str()) {
            Ok(encoded_token) => HttpResponse::Ok().body(encoded_token),
            Err(e) => HttpResponse::InternalServerError().body(format!("Error creating jwt: {}", e)),
        },
        Err(e) => HttpResponse::UnprocessableEntity().body(format!("{}", e)),
    }
}

macro_rules! data_endpoint {
    ($endp_name: expr, $mthd_name: ident, $typ: ty, $act: ident) => {
         #[post($endp_name)]
         async fn $mthd_name(req: HttpRequest, body: web::Json<$typ>, shop_repo: web::Data<Arc<SupermarketRepository>>) -> impl Responder {
            let body = body.into_inner();
            let action_fut = Action::from_req(&req, body);
            utils::handle_query_and_auth(&req, action_fut.and_then(|act| shop_repo.$act(act)).await, "manager")
        }
    };
}

macro_rules! data_endpoint_no_action {
    ($endp_name: expr, $mthd_name: ident, $act: ident) => {
        #[get($endp_name)]
        async fn $mthd_name(req: HttpRequest, shop_repo: web::Data<Arc<SupermarketRepository>>) -> impl Responder {
            utils::handle_query_and_auth(&req, shop_repo.$act().await, "manager")
        }
    };
    ($endp_name: expr, $mthd_name: ident, $typ: ty, $act: ident) => {
         #[post($endp_name)]
         async fn $mthd_name(req: HttpRequest, body: web::Json<$typ>, shop_repo: web::Data<Arc<SupermarketRepository>>) -> impl Responder {
            utils::handle_query_and_auth(&req, shop_repo.$act(body.into_inner()).await, "manager")
        }
    };
}

data_endpoint!("/employee", employee, ShopEmployee, handle_employee);

data_endpoint!("/client_card", client_card, ClientCard, handle_client_card);

data_endpoint!("/manufacturer", manufacturer, Manufacturer, handle_manufacturer);

data_endpoint!("/product", product, Product, handle_product);

data_endpoint!("/owned_product", owned_product, OwnedProduct, handle_owned_product);

data_endpoint!("/category", category, Category, handle_category);

data_endpoint!("/waybill", waybill, Waybill, handle_waybill);

data_endpoint!("/return_agreement", return_agreement, ReturnAgreement, handle_return_agreement);

data_endpoint_no_action!("/create_receipt", create_receipt, CreateReceipt, handle_create_receipt);

data_endpoint_no_action!("/delete_receipt", delete_receipt, Receipt, handle_delete_receipt);

data_endpoint_no_action!("/utils/get_all_categories", utils_get_all_categories, get_all_categories);

data_endpoint_no_action!("/utils/get_all_client_cards", utils_get_all_client_cards, get_all_client_cards);

data_endpoint_no_action!("/utils/get_all_products", utils_get_all_products, get_all_products);

#[post("/update_sale")]
pub async fn update_sale(req: HttpRequest, body: web::Json<Sale>, shop_repo: web::Data<Arc<SupermarketRepository>>) -> impl Responder {
    utils::handle_query_and_auth(&req, shop_repo.handle_update_sale(body.into_inner()).await, "cashier")
}

#[post("/manager_query/{query_name}")]
pub async fn manager_query(req: HttpRequest, body: web::Json<serde_json::Value>, shop_repo: web::Data<Arc<SupermarketRepository>>) -> impl Responder {
    let query_name = req.match_info().get("query_name").unwrap();
    utils::handle_query_and_auth(&req, shop_repo.handle_manager_query(query_name, body.into_inner()).await.map(Some), "manager")
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
        .service(update_sale)
        .service(manager_query)
        .service(utils_get_all_categories)
        .service(utils_get_all_products)
        .service(utils_get_all_client_cards);
    let admin_scope = web::scope("/admin").service(create_new_user).service(get_user);
    server_cfg
        .service(healthcheck)
        .service(api_scope)
        .service(admin_scope)
        .service(login);
}
