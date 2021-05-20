pub mod jwt;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

fn cashier_or_manager(role: &String) -> Result<(), ValidationError> {
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
    pub user_role: String, // manager/cashier
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub full_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
pub struct UpdateProfile {
    pub full_name: Option<String>,
    #[validate(url)]
    pub image_link: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticatedUser {
    pub username: String,
    pub user_role: String,
}

#[derive(Serialize, Deserialize, Debug, Validate, sqlx::FromRow)]
pub struct Manufacturer {
    pub manufacturer_id: usize,
    pub contract_id: usize,
    pub contract_sign_date: NaiveDateTime,
    pub contract_end_date: NaiveDateTime,
    pub manufacturer_name: String,
    pub country: String,
    pub addr_city: String,
    pub addr_street: String,
    #[validate(length(equal = 5))]
    pub addr_postal: String,
    #[validate(length(equal = 13))]
    pub tel_num: String,
}
