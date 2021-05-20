use crate::models::AuthenticatedUser;
use crate::utils;
use color_eyre::{Report, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppRequest {
    pub action: String, // create, update, delete
    pub target: String, // manufacturer, employee, product,owned_product, category, waybill, return_agreement, receipt, client_card
}
