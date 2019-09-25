#![feature(proc_macro_hygiene, decl_macro)]

extern crate juniper;
extern crate juniper_rocket;
extern crate regular_dicers_backend;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use std;
use std::collections::HashMap;

use diesel::Connection;
use diesel::pg::PgConnection;
use rocket::{Config, response::content, State};
use rocket::config::{Environment, Value};

use regular_dicers_backend::database::mock::Mockable;
use regular_dicers_backend::db::PrimaryDb;
use regular_dicers_backend::graphql::{MutationRoot, QueryRoot};
use regular_dicers_backend::graphql::schema::Constraints;
use regular_dicers_backend::models::CocktailIngredient;
use regular_dicers_backend::routes;

fn main() -> Result<(), diesel::result::Error> {
    let database_default = "postgres://postgres:password@localhost/regular_dicers_backend".to_string();
    let connection = PgConnection::establish(
        std::env::var("DATABASE_URL")
            .unwrap_or(database_default)
            .as_ref())
        .expect("Failed to create database connection");
    let _ = populate_database(100, &connection)?;

    let server = if std::env::var("ROCKET_ENV").unwrap_or("dev".to_string()) == "prod" {
        let mut db_config = HashMap::new();
        let mut databases = HashMap::new();
        let url = if let Ok(url) = std::env::var("DATABASE_URL") {
            url
        } else {
            panic!("DATABASE_URL has to be provided in a production setting")
        };
        db_config.insert("url", Value::from(url));
        databases.insert("primary_db", Value::from(db_config));

        rocket::custom(Config::build(Environment::Production)
            .extra("databases", databases)
            .finalize()
            .unwrap())
    } else {
        rocket::ignite()
    };

    server
        .attach(PrimaryDb::fairing())
        .manage(routes::Schema::new(QueryRoot, MutationRoot))
        .mount("/",
               routes![
                routes::graphiql,
                routes::get_graphql_handler,
                routes::post_graphql_handler,
               ])
        .launch();

    Ok(())
}

fn populate_database(n: u32, connection: &diesel::PgConnection) -> Result<(), diesel::result::Error> {
    for _ in 0..n {
        let user = regular_dicers_backend::database::models::UserI::mock(None).unwrap().insert(&connection)?;

        let chat = regular_dicers_backend::database::models::ChatI::mock(None).unwrap().insert(&connection)?;

        let mut chat_user_data = HashMap::new();
        chat_user_data.insert("chat_id".to_string(), chat.id);
        chat_user_data.insert("user_id".to_string(), user.id);
        regular_dicers_backend::database::models::ChatUserI::mock(Some(chat_user_data)).unwrap().insert(&connection)?;

        let mut event_data = HashMap::new();
        event_data.insert("chat_id".to_string(), chat.id);
        let event = regular_dicers_backend::database::models::EventI::mock(Some(event_data)).unwrap().insert(&connection)?;

        let mut event_user_data = HashMap::new();
        event_user_data.insert("user_id".to_string(), user.id);
        event_user_data.insert("event_id".to_string(), event.id);
        let event_user = regular_dicers_backend::database::models::EventUserI::mock(Some(event_user_data)).unwrap().insert(&connection)?;

        if event_user.attends {
            let mut user_roll_data = HashMap::new();
            user_roll_data.insert("event_user_id".to_string(), event_user.id);
            regular_dicers_backend::database::models::UserRollI::mock(Some(user_roll_data)).unwrap().insert(&connection)?;
        }

        let mut message_data = HashMap::new();
        message_data.insert("chat_id".to_string(), chat.id);
        message_data.insert("user_id".to_string(), user.id);
        regular_dicers_backend::database::models::MessageI::mock(Some(message_data)).unwrap().insert(&connection)?;
    }

    Ok(())
}
