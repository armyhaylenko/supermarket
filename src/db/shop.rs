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
            .and_then(|extracted_query: Query<HashMap<String, String>>| futures::future::ready(Ok(extracted_query.0)))
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
            Action::Create(cc) => {
                let sql = include_str!("../../sql/client_card/create_client_card.sql");
                cc.validate()?;
                sqlx::query_as::<_, ClientCard>(sql)
                    .bind(cc.first_name)
                    .bind(cc.last_name)
                    .bind(cc.patronymic)
                    .bind(cc.phone_num)
                    .bind(cc.addr_city)
                    .bind(cc.addr_street)
                    .bind(cc.addr_postal)
                    .bind(PgMoney::from_decimal(cc.discount_rate, 2))
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err))
            }
            Action::Delete(cc) => {
                let sql = include_str!("../../sql/client_card/delete_client_card.sql");
                sqlx::query_as::<_, ClientCard>(sql)
                    .bind(cc.card_id)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err))
            }
            Action::Update(cc) => {
                let sql = include_str!("../../sql/client_card/update_client_card.sql");
                cc.validate()?;
                sqlx::query_as::<_, ClientCard>(sql)
                    .bind(cc.card_id)
                    .bind(cc.first_name)
                    .bind(cc.last_name)
                    .bind(cc.patronymic)
                    .bind(cc.phone_num)
                    .bind(cc.addr_city)
                    .bind(cc.addr_street)
                    .bind(cc.addr_postal)
                    .bind(PgMoney::from_decimal(cc.discount_rate, 2))
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err))
            }
        }
    }

    pub async fn handle_manufacturer(&self, action: Action<Manufacturer>) -> Result<Option<Manufacturer>> {
        match action {
            Action::Create(m) => {
                let sql = include_str!("../../sql/manufacturer/create_manufacturer.sql");
                m.validate()?;
                sqlx::query_as::<_, Manufacturer>(sql)
                    .bind(m.contract_id)
                    .bind(m.contract_sign_date)
                    .bind(m.contract_end_date)
                    .bind(m.manufacturer_name)
                    .bind(m.country)
                    .bind(m.addr_city)
                    .bind(m.addr_street)
                    .bind(m.addr_postal)
                    .bind(m.tel_num)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err))
            }
            Action::Delete(m) => {
                let sql = include_str!("../../sql/manufacturer/delete_manufacturer.sql");
                sqlx::query_as::<_, Manufacturer>(sql)
                    .bind(m.manufacturer_id)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err))
            }
            Action::Update(m) => {
                let sql = include_str!("../../sql/manufacturer/update_manufacturer.sql");
                m.validate()?;
                sqlx::query_as::<_, Manufacturer>(sql)
                    .bind(m.manufacturer_id)
                    .bind(m.contract_id)
                    .bind(m.contract_sign_date)
                    .bind(m.contract_end_date)
                    .bind(m.manufacturer_name)
                    .bind(m.country)
                    .bind(m.addr_city)
                    .bind(m.addr_street)
                    .bind(m.addr_postal)
                    .bind(m.tel_num)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err))
            }
        }
    }

    pub async fn handle_product(&self, action: Action<Product>) -> Result<Option<Product>> {
        match action {
            Action::Create(p) => {
                let sql = include_str!("../../sql/product/create_product.sql");
                p.validate()?;
                sqlx::query_as::<_, Product>(sql)
                    .bind(p.product_name)
                    .bind(p.descr)
                    .bind(p.category_id)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err))
            }
            Action::Delete(p) => {
                let sql = include_str!("../../sql/product/delete_product.sql");
                sqlx::query_as::<_, Product>(sql)
                    .bind(p.product_id)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err))
            }
            Action::Update(p) => {
                let sql = include_str!("../../sql/product/delete_product.sql");
                p.validate()?;
                sqlx::query_as::<_, Product>(sql)
                    .bind(p.product_id)
                    .bind(p.product_name)
                    .bind(p.descr)
                    .bind(p.category_id)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err))
            }
        }
    }

    pub async fn handle_owned_product(&self, action: Action<OwnedProduct>) -> Result<Option<OwnedProduct>> {
        async fn query(repo: &SupermarketRepository, sql: &str, p: OwnedProduct) -> Result<Option<OwnedProduct>> {
            sqlx::query_as::<_, OwnedProduct>(sql)
                .bind(p.product_upc)
                .bind(p.product_id)
                .bind(p.sale_price)
                .bind(p.units_in_stock)
                .bind(p.is_on_sale)
                .bind(p.on_sale_product_upc)
                .fetch_optional(&*repo.pool)
                .await
                .map_err(|err| Report::new(err))
        }
        match action {
            Action::Create(p) => {
                let sql = include_str!("../../sql/owned_product/create_owned_product.sql");
                p.validate()?;
                query(&self, sql, p).await
            }
            Action::Delete(p) => {
                let sql = include_str!("../../sql/owned_product/delete_owned_product.sql");
                sqlx::query_as::<_, OwnedProduct>(sql)
                    .bind(p.product_upc)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err))
            }
            Action::Update(p) => {
                let sql = include_str!("../../sql/owned_product/delete_owned_product.sql");
                p.validate()?;
                query(&self, sql, p).await
            }
        }
    }

    // TODO
    // pub async fn handle_category(&self, action: Action<Category>) -> Result<Option<Category>> {
    //     match action {
    //         Action::Create(c) => {}
    //         Action::Delete(c) => Ok(None),
    //         Action::Update(c) => {}
    //     }
    // }

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
