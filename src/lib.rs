#![feature(option_result_contains)]
#![feature(array_map)]
#[macro_use]
extern crate validator_derive;
extern crate rust_decimal;

pub mod config;
pub mod db;
pub mod handlers;
pub mod models;
mod utils;
