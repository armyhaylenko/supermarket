use crate::models::*;
use actix_web::{web, web::Query, FromRequest, HttpRequest};
use color_eyre::{Report, Result};
use futures::{StreamExt, TryFutureExt};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use validator::Validate;
use serde_json::Value;

pub enum Action<T> {
    Create(T),
    Delete(T),
    Update(T),
}

macro_rules! build_query {
        ($self: ident, $qpath: expr, $typ: ty) => {
            sqlx::query_as::<_, $typ>(include_str!($qpath))
                    .fetch_all(&*$self.pool)
                    .await
                    .map_err(|err| Report::new(err))
                    .and_then(|_models| serde_json::to_string(&_models)
                                                        .map_err(|err| Report::new(err)))
        };
        ($self: ident, $json: expr, $qpath: expr, $typ: ty $(,$filter: expr)*) => {
            sqlx::query_as::<_, $typ>(include_str!($qpath))
                    $(
                    .bind(&$json.get($filter)
                        .ok_or(Report::msg(concat!("Invalid filter provided: could not find field ", $filter)))
                        .and_then(|j: &Value|
                                j.as_str()
                                 .ok_or(Report::msg(concat!("Invalid filter provided: could not convert field ", $filter, " to string")))
                            )?
                        )  // only strings are present in tech reqs
                    )*
                    .fetch_all(&*$self.pool)
                    .await
                    .map_err(|err| Report::new(err))
                    .and_then(|_models| serde_json::to_string(&_models)
                                                        .map_err(|err| Report::new(err)))
        }
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
                    .bind(e.salary)
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
                    .bind(e.salary)
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
                    .bind(cc.discount_rate)
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
                    .bind(cc.discount_rate)
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

    pub async fn handle_category(&self, action: Action<Category>) -> Result<Option<Category>> {
        match action {
            Action::Create(c) => {
                let sql = include_str!("../../sql/category/create_category.sql");
                c.validate()?;
                sqlx::query_as(sql)
                    .bind(c.category_name)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err))
            }
            Action::Delete(_) => Ok(None),
            Action::Update(c) => {
                let sql = include_str!("../../sql/category/update_category.sql");
                c.validate()?;
                sqlx::query_as(sql)
                    .bind(c.category_id)
                    .bind(c.category_name)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err))
            }
        }
    }

    pub async fn handle_waybill(&self, action: Action<Waybill>) -> Result<Option<Waybill>> {
        match action {
            Action::Create(w) => {
                let sql = include_str!("../../sql/waybill/create_waybill.sql");
                w.validate()?;
                sqlx::query_as(sql)
                    .bind(w.waybill_date)
                    .bind(w.base_price)
                    .bind(w.waybill_sum)
                    .bind(w.qty)
                    .bind(w.product_upc)
                    .bind(w.manufacturer_id)
                    .bind(w.empl_id)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err))
            }
            Action::Delete(w) => {
                let sql = include_str!("../../sql/waybill/delete_waybill.sql");
                sqlx::query_as(sql)
                    .bind(w.waybill_id)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err))
            }
            Action::Update(_) => Ok(None),
        }
    }

    pub async fn handle_return_agreement(&self, action: Action<ReturnAgreement>) -> Result<Option<ReturnAgreement>> {
        match action {
            Action::Create(ra) => {
                let sql = include_str!("../../sql/return_agreement/create_return_agreement.sql");
                ra.validate()?;
                sqlx::query_as(sql)
                    .bind(ra.sign_date)
                    .bind(ra.qty)
                    .bind(ra.return_agreement_sum)
                    .bind(ra.product_upc)
                    .bind(ra.manufacturer_id)
                    .bind(ra.empl_id)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err))
            }
            Action::Delete(ra) => {
                let sql = include_str!("../../sql/return_agreement/delete_return_agreement.sql");
                sqlx::query_as(sql)
                    .bind(ra.sign_date)
                    .fetch_optional(&*self.pool)
                    .await
                    .map_err(|err| Report::new(err))
            }
            Action::Update(_) => Ok(None),
        }
    }

    pub async fn handle_create_receipt(&self, create_receipt: CreateReceipt) -> Result<Option<Receipt>> {
        let receipt_sum = create_receipt.sales.iter().fold(Decimal::new(0, 2), |prev_sum, sale| {
            prev_sum + (sale.price * Decimal::new(sale.qty as i64, 0))
        });
        let receipt_date = create_receipt.receipt_date;
        let vat = receipt_sum * Decimal::new(20, 2) / Decimal::new(120, 2);
        let receipt = Receipt {
            receipt_id: None,
            receipt_date,
            receipt_sum,
            vat,
            client_card_id: create_receipt.client_card_id,
        };
        let sql = include_str!("../../sql/receipt/create_receipt.sql");
        let receipt: Receipt = sqlx::query_as::<_, Receipt>(sql)
            .bind(receipt.receipt_date)
            .bind(receipt.receipt_sum)
            .bind(receipt.vat)
            .bind(receipt.client_card_id)
            .fetch_one(&*self.pool)
            .await
            .map_err(|err| Report::new(err))?;

        let sale_task_queue = futures::stream::futures_unordered::FuturesUnordered::new();
        for sale in create_receipt.sales.into_iter() {
            let query_fut = sqlx::query(include_str!("../../sql/sale/create_sale.sql"))
                .bind(receipt.receipt_id)
                .bind(sale.product_upc)
                .bind(sale.qty)
                .bind(sale.price)
                .fetch_optional(&*self.pool);
            sale_task_queue.push(query_fut);
        }
        let _ = sale_task_queue.collect::<Vec<_>>().await;

        Ok(Some(receipt))
    }

    pub async fn handle_delete_receipt(&self, receipt: Receipt) -> Result<Option<Receipt>> {
        sqlx::query_as(include_str!("../../sql/receipt/delete_receipt.sql"))
            .bind(receipt.receipt_id)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|err| Report::new(err))
    }

    pub async fn handle_update_sale(&self, sale: Sale) -> Result<Option<Sale>> {
        sqlx::query_as(include_str!("../../sql/sale/update_sale.sql"))
            .bind(sale.receipt_id)
            .bind(sale.product_upc)
            .bind(sale.price)
            .bind(sale.qty)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|err| Report::new(err))
    }


    pub async fn handle_manager_query(
        &self,
        query_name: &str,
        json: serde_json::Value,
    ) -> Result<String> {
        match query_name {
            "get_all_cashiers" => build_query!(self, json, "../../sql/manager_queries/get_all_cashiers.sql", ShopEmployee),
            "get_all_products_by_category" => build_query!(self, json, "../../sql/manager_queries/get_all_products_by_category.sql", Product, "category_name"),
            _ => Ok(String::new()),
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

    pub async fn get_all_categories(&self) -> Result<Option<String>> {
        build_query!(self, "../../sql/utils/get_all_categories.sql", Category).map(Some)
    }
    pub async fn get_all_client_cards(&self) -> Result<Option<String>> {
        build_query!(self, "../../sql/utils/get_all_client_cards.sql", ClientCard).map(Some)
    }
    pub async fn get_all_products(&self) -> Result<Option<String>> {
        build_query!(self, "../../sql/utils/get_all_products.sql", Product).map(Some)
    }
}

#[derive(Serialize, Debug, Deserialize, sqlx::FromRow)]
struct I32Result {
    // this exists only because sqlx needs a type that implements FromRow to deserialize the db output
    max_empl_id: i32,
}
