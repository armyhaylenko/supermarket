use crate::models::ShopEmployee;
use actix_web::web::Form;
use actix_web::{get, HttpRequest, HttpResponse, Responder};
use color_eyre::{Report, Result};
use sqlx::PgPool;
use std::str::FromStr;
use std::sync::Arc;

pub enum Action<T> {
    Create(T),
    Delete(T),
    Update(T),
}

pub struct SupermarketRepository {
    pool: Arc<PgPool>,
}

impl SupermarketRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
    pub async fn handle_employee(&self, action: Action<ShopEmployee>) -> Result<Option<ShopEmployee>> {
        match action {
            Action::Create(e) => {
                let sql = include_str!("../../sql/create_employee.sql");
                let query_result: Result<Option<ShopEmployee>> = sqlx::query_as::<_, ShopEmployee>(sql)
                    .bind(e.first_name)
                    .bind(e.last_name)
                    .bind(e.patronymic)
                    .bind(e.user_role)
                    .bind(e.salary)
                    .bind(e.join_date)
                    .bind(e.phone_num)
                    .bind(e.addr_city)
                    .bind(e.addr_street)
                    .bind(e.addr_postal)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err));
                query_result
            }
            Action::Delete(e) => Ok(Some(e)),
            Action::Update(e) => Ok(Some(e)),
        }
    }
}

// pub async fn get_manufacturer() -> impl Responder {
//     HttpResponse::Ok().finish()
// }
//
// #[post(/employee)]
// pub async fn get_employee(req: HttpRequest, shop_repo: Arc<>)
