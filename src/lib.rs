#![feature(decl_macro, proc_macro_hygiene)]
extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate juniper;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rand;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate rocket_http;

pub mod database;
pub mod models;
pub mod graphql;
pub mod routes;
pub mod db;
