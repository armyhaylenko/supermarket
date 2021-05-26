mod test_endpoints;

use crate::config::crypto::CryptoService;
use crate::db::{Action, SupermarketRepository, UserRepository};
use crate::models::{NewUser, ShopEmployee};
use crate::utils::has_auth_level;
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
    let action = Action::from_req(&req, emp);
    if !has_auth_level(&req, "manager".to_owned()) {
        HttpResponse::Unauthorized()
            .body("The authorization header is not present or is malformed, or the user does not have access to this resource.")
    } else {
        match action.and_then(|act| shop_repo.handle_employee(act)).await {
            Ok(ref maybe_employee) => match serde_json::to_string::<Option<ShopEmployee>>(maybe_employee) {
                Ok(query_result) => HttpResponse::Ok().body(query_result),
                Err(e) => HttpResponse::InternalServerError().body(format!("Unable to convert query result to json struct: {}", e)),
            },
            Err(e) => HttpResponse::InternalServerError().body(format!("The database was not able to process employee request: {}", e)),
        }
    }
}

pub fn init_app_config(server_cfg: &mut ServiceConfig) -> () {
    let tests_scope = web::scope("/tests").service(test_endpoints::get_most_recent_employee);
    let api_scope = web::scope("/api").service(tests_scope).service(employee);
    let admin_scope = web::scope("/admin").service(create_new_user).service(get_user);
    server_cfg.service(healthcheck).service(api_scope).service(admin_scope);
}
