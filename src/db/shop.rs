use crate::models::*;
use actix_web::{web, web::Query, FromRequest, HttpRequest};
use color_eyre::{Report, Result};
use futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use sqlx::postgres::types::PgMoney;
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use validator::Validate;

pub enum Action<T> {
    Create(T),
    Delete(T),
    Update(T),
}

impl<T> Action<T> {
    pub async fn from_req(req: &HttpRequest, body: T) -> Result<Action<T>> {
        let hash_map: Result<HashMap<String, String>> = web::Query::<HashMap<String, String>>::extract(&req)
            .and_then(|extr_query: Query<HashMap<String, String>>| futures::future::ready(Ok(extr_query.0)))
            .await
            .map_err(|e| Report::msg(format!("{:?}", e)));

        let action = hash_map.and_then(|map| {
            let param_value_result = map.get(&"action".to_owned()).ok_or(Report::msg("action param not found"));
            param_value_result.and_then(|param_value| match param_value.as_str() {
                "create" => Ok(Action::<T>::Create(body)),
                "delete" => Ok(Action::<T>::Delete(body)),
                "update" => Ok(Action::<T>::Update(body)),
                other_act => Err(Report::msg(format!("Wrong action provided in query string: {}", other_act))),
            })
        });
        action
    }
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
                let sql = include_str!("../../sql/employee/create_employee.sql");
                e.validate()?;
                sqlx::query_as::<_, ShopEmployee>(sql)
                    .bind(e.first_name)
                    .bind(e.last_name)
                    .bind(e.patronymic)
                    .bind(e.user_role)
                    .bind(PgMoney::from_decimal(e.salary, 2))
                    .bind(e.join_date)
                    .bind(e.phone_num)
                    .bind(e.addr_city)
                    .bind(e.addr_street)
                    .bind(e.addr_postal)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err))
            }
            Action::Delete(e) => {
                let sql = include_str!("../../sql/employee/delete_employee.sql");
                sqlx::query_as::<_, ShopEmployee>(sql)
                    .bind(e.empl_id)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err))
            }
            Action::Update(e) => {
                let sql = include_str!("../../sql/employee/update_employee.sql");
                e.validate()?;
                sqlx::query_as::<_, ShopEmployee>(sql)
                    .bind(e.empl_id)
                    .bind(e.first_name)
                    .bind(e.last_name)
                    .bind(e.patronymic)
                    .bind(e.user_role)
                    .bind(PgMoney::from_decimal(e.salary, 2))
                    .bind(e.join_date)
                    .bind(e.phone_num)
                    .bind(e.addr_city)
                    .bind(e.addr_street)
                    .bind(e.addr_postal)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err))
            }
        }
    }

    pub async fn handle_client_card(&self, action: Action<ClientCard>) -> Result<Option<ClientCard>> {
        match action {
            Action::Create(cc) => Ok(Some(cc)),
            Action::Delete(cc) => Ok(Some(cc)), // TODO!
            Action::Update(cc) => Ok(Some(cc)), // TODO!
        }
    }

    pub async fn get_most_recent_employee(&self) -> Result<Option<i32>> {
        let sql = include_str!("../../sql/employee/tests/get_most_recent_employee.sql");
        sqlx::query_as::<_, I32Result>(sql)
            .fetch_optional(&*self.pool)
            .await
            .map(|res| res.map(|i32_r| i32_r.max_empl_id))
            .map_err(|e| Report::new(e))
    }
}

#[derive(Serialize, Debug, Deserialize, sqlx::FromRow)]
struct I32Result {
    max_empl_id: i32,
}
