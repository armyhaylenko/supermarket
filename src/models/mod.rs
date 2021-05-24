use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::postgres::types::PgMoney;
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
    pub user_role: String, // manager/cashier; no validation here because this struct is intended to only be retrieved from db,
    // and not used for inserts / updates
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
    pub manufacturer_id: Option<i32>,
    #[validate(length(equal = 32))]
    pub contract_id: String,
    pub contract_sign_date: NaiveDateTime,
    pub contract_end_date: NaiveDateTime,
    #[validate(length(max = 200))]
    pub manufacturer_name: String,
    pub country: String,
    pub addr_city: String,
    pub addr_street: String,
    #[validate(length(equal = 5))]
    pub addr_postal: String,
    #[validate(length(equal = 13))]
    pub tel_num: String,
}

#[derive(Serialize, Deserialize, Debug, Validate, sqlx::FromRow)]
pub struct ShopEmployee {
    pub empl_id: Option<i32>,
    #[validate(length(max = 50))]
    pub first_name: String,
    #[validate(length(max = 100))]
    pub last_name: String,
    #[validate(length(max = 50))]
    pub patronymic: String,
    #[validate(custom = "cashier_or_manager")]
    #[sqlx(rename = "position")] // position in supermarket db, user_role in code - it's cashier or manager anyway
    pub user_role: String,
    pub salary: i64,
    pub join_date: NaiveDateTime,
    #[validate(length(equal = 13))]
    pub phone_num: String,
    pub addr_city: String,
    pub addr_street: String,
    #[validate(length(equal = 5))]
    pub addr_postal: String,
}

#[derive(Serialize, Deserialize, Debug, Validate, sqlx::FromRow)]
pub struct Product {
    pub product_id: Option<i32>,
    #[validate(length(max = 300))]
    pub product_name: String,
    #[validate(length(max = 500))]
    pub descr: String,
    pub category_id: usize,
}

#[derive(Serialize, Deserialize, Debug, Validate, sqlx::FromRow)]
pub struct OwnedProduct {
    #[validate(length(equal = 12))]
    pub product_upc: String,
    pub product_id: usize,
    pub sale_price: i64,
    pub units_in_stock: usize,
    pub is_on_sale: bool,
    pub on_sale_product_upc: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate, sqlx::FromRow)]
pub struct Category {
    pub category_id: Option<i32>,
    #[validate(length(max = 200))]
    pub category_name: String,
}

// pub struct Waybill {
//     pub
// }
