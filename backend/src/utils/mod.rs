use crate::models::auth::AuthenticatedUser;
use actix_web::http::{header, header::ToStrError};
use actix_web::{HttpRequest, HttpResponse};
use color_eyre::{Report, Result};
use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::debug;

pub fn decode_token_to_user(token: &str, secret_key: &str) -> Result<AuthenticatedUser> {
    let decoding_key = DecodingKey::from_base64_secret(secret_key);
    let decoded_token: Result<AuthenticatedUser> = decoding_key
        .and_then(|key| {
            let mut validation = Validation::new(Algorithm::default());
            validation.validate_exp = false;
            let r = decode::<AuthenticatedUser>(&token, &key, &validation);
            debug!("{:?}", r);
            r
        })
        .map(|t_data| t_data.claims)
        .map_err(|e| Report::new(e));
    decoded_token
}

pub fn encode_user_to_token(user: &AuthenticatedUser, secret_key: &str) -> Result<String> {
    jsonwebtoken::encode(&Header::default(), &user, &EncodingKey::from_base64_secret(secret_key)?).map_err(Report::new)
}

pub fn handle_auth(req: &HttpRequest) -> Result<String> {
    let auth: Option<Result<&str, ToStrError>> = req.headers().get(header::AUTHORIZATION).map(|header_value| header_value.to_str());

    match auth {
        Some(maybe_token) => maybe_token.map_err(|e| color_eyre::Report::new(e)).and_then(|token| {
            std::env::var("JWT_DECODING_KEY")
                .map_err(|e| color_eyre::Report::new(e))
                .and_then(|key| decode_token_to_user(token.replace("Bearer ", "").as_str(), key.as_str()).map(|usr| usr.user_role))
        }),
        None => Err(Report::msg("No authorization header was provided.")),
    }
}

pub fn has_auth_level(req: &HttpRequest, auth_level: String) -> bool {
    let auth = handle_auth(&req);

    if auth_level.eq("both") {
        true
    }
    else {
        auth.contains(&auth_level)
    }
}

fn query_result_to_http_response<'a, T: Serialize + Deserialize<'a>>(r: Result<Option<T>>) -> HttpResponse {
    match r {
        Ok(ref maybe_object) => match serde_json::to_string(maybe_object) {
            Ok(query_res) => HttpResponse::Ok().body(query_res),
            Err(e) => HttpResponse::InternalServerError().body(format!("Unable to convert query result to json struct: {}", e)),
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("The database was not able to process request: {}", e)),
    }
}

pub fn handle_query_and_auth<'a, T: Serialize + Deserialize<'a>>(
    req: &HttpRequest,
    query_result: Result<Option<T>>,
    auth_level: &str,
) -> HttpResponse {
    if !has_auth_level(&req, auth_level.to_owned()) {
        HttpResponse::Unauthorized()
            .body("The authorization header is not present or is malformed, or the user does not have access to this resource.")
    } else {
        query_result_to_http_response(query_result)
    }
}
