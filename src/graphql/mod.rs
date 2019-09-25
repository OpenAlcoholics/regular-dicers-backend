use crate::db::PrimaryDb;

pub mod schema;

pub struct Context {
    pub connection: PrimaryDb
}

impl juniper::Context for Context {}

pub struct MutationRoot;

pub struct QueryRoot;
