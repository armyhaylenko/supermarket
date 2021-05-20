use crate::config::crypto::CryptoService;
use crate::db::UserRepository;
use crate::models::NewUser;
use crate::utils;
use actix_web::http::header::ToStrError;
use actix_web::http::{HeaderMap, HeaderValue};
use actix_web::{
    get,
    http::header,
    post, web,
    web::{Form, ServiceConfig},
    HttpRequest, HttpResponse, Responder,
};
use std::sync::Arc;
use tracing::debug;
use validator::Validate;

#[get("/healthcheck")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("Working!")
}
#[post("/admin/create_user")]
pub async fn create_new_user(
    req: HttpRequest,
    form: Form<NewUser>,
    user_repo: web::Data<Arc<UserRepository>>,
    crypto_svc: web::Data<Arc<CryptoService>>,
) -> impl Responder {
    let maybe_role = utils::handle_auth(&req);

    if !maybe_role.contains::<String>(&"manager".to_owned()) {
        HttpResponse::Unauthorized().body("The authorization header is not present or is malformed.")
    } else {
        let new_user_unvalidated = form.0;

        match new_user_unvalidated.validate() {
            Ok(_) => match user_repo.create_user(new_user_unvalidated, &*crypto_svc).await {
                Ok(usr) => HttpResponse::Ok().body(format!("User {} was successfully added", usr.username)),
                Err(e) => HttpResponse::UnprocessableEntity().body(format!("{:?}", e)),
            },
            Err(e) => HttpResponse::UnprocessableEntity().body(format!("{:?}", e)),
        }
    }
}

#[get("/admin/user/{username}")]
pub async fn get_user(req: HttpRequest, user_repo: web::Data<Arc<UserRepository>>) -> impl Responder {
    let username = String::from(req.match_info().get("username").unwrap());

    let maybe_role = utils::handle_auth(&req);
    if !maybe_role.contains::<String>(&String::from("manager")) {
        HttpResponse::Unauthorized().body("The authorization header is not present or is malformed.")
    } else {
        match user_repo.get_user_by_username(username.clone()).await {
            Ok(maybe_user) => match maybe_user {
                Some(usr) => HttpResponse::Ok().body(format!(
                    "{:?}",
                    serde_json::to_string(&usr).expect(
                        "The user was malformed in the database, or the query didn't work \
                 correctly"
                    )
                )),
                None => HttpResponse::NotFound().body(format!("Could not find user {}", username.clone())),
            },
            Err(e) => HttpResponse::InternalServerError().body(format!(
                "The database was not able \
        to process get_user request: {:?}",
                e
            )),
        }
    }
}

pub fn init_app_config(server_cfg: &mut ServiceConfig) -> () {
    server_cfg.service(healthcheck).service(create_new_user).service(get_user);
}
