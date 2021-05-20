use crate::models::AuthenticatedUser;
use actix_web::http::{header, header::ToStrError};
use actix_web::HttpRequest;
use color_eyre::{Report, Result};
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use tracing::debug;

pub fn decode_token_to_user(token: &str, secret_key: &str) -> Result<AuthenticatedUser> {
    let decoding_key = DecodingKey::from_base64_secret(secret_key);
    let decoded_token: Result<AuthenticatedUser> = decoding_key
        .and_then(|key| {
            let r = decode::<AuthenticatedUser>(&token, &key, &Validation::new(Algorithm::default()));
            debug!("{:?}", r);
            r
        })
        .map(|t_data| t_data.claims)
        .map_err(|e| Report::new(e));
    decoded_token
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
