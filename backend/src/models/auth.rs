use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::ValidationError;

pub fn cashier_or_manager(role: &String) -> Result<(), ValidationError> {
    let role_str = &role[..];
    match role_str {
        "cashier" => Ok(()),
        "manager" => Ok(()),
        _ => Err(ValidationError::new("Neither cashier nor manager")),
    }
}

#[derive(Serialize, sqlx::FromRow, Debug)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub user_role: String, // manager/cashier; no validation here because this struct is intended to only be retrieved from db,
    // and not used for inserts / updates
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub full_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub salt: String,
}

#[derive(Deserialize, Validate, Debug)]
pub struct NewUser {
    #[validate(length(max = 150))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(max = 24))]
    pub password: String,
    #[validate(custom = "cashier_or_manager")]
    pub user_role: String,
}

#[derive(Deserialize, Validate, Debug)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticatedUser {
    pub username: String,
    pub user_role: String,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct HashAndSalt {
    pub password_hash: String,
    pub salt: String,
}
