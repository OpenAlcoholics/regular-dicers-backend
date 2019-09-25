use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::database::schema::{self, *};
use crate::graphql::schema::Constraints;
use crate::models;

type DieselResult<T> = Result<T, diesel::result::Error>;

#[derive(Associations, Debug, Identifiable, Queryable)]
pub struct Chat {
    pub id: i32,
    pub chat_type: i16,
    pub telegram_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub current_keyboard: i16,
    pub spam_detection: bool,
}

#[derive(AsChangeset, Debug, Insertable)]
#[table_name = "chats"]
pub struct ChatI {
    pub chat_type: i16,
    pub telegram_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub current_keyboard: i16,
    pub spam_detection: bool,
}

#[derive(Associations, Debug, Identifiable, Queryable)]
#[belongs_to(Chat)]
#[belongs_to(User, foreign_key = "user_id")]
pub struct ChatUser {
    pub id: i32,
    pub user_id: i32,
    pub chat_id: i32,
    pub spamming: bool,
    pub muted: bool,
    pub admin: bool,
}

#[derive(AsChangeset, Debug, GraphQLInputObject, Insertable)]
#[table_name = "chat_users"]
pub struct ChatUserI {
    pub user_id: i32,
    pub chat_id: i32,
    pub spamming: bool,
    pub muted: bool,
    pub admin: bool,
}

// unique(chat_id, active)
#[derive(Associations, Debug, Identifiable, Queryable)]
pub struct Event {
    pub id: i32,
    pub chat_id: i32,
    pub timestamp: NaiveDateTime,
    pub active: bool,
}

#[derive(AsChangeset, Debug, Insertable)]
#[table_name = "events"]
pub struct EventI {
    pub chat_id: i32,
    pub timestamp: NaiveDateTime,
    pub active: bool,
}

#[derive(AsChangeset, Associations, Debug, Identifiable, Queryable)]
pub struct EventUser {
    pub id: i32,
    pub(crate) event_id: i32,
    pub(crate) user_id: i32,
    pub attends: bool,
}

#[derive(AsChangeset, Associations, Debug, Identifiable, Queryable)]
#[belongs_to(EventUser)]
pub struct UserRoll {
    pub id: i32,
    pub event_user_id: i32,
    pub callback_query_id: String,
    pub jumbo: bool,
    pub alcoholic: bool,
    pub roll: i32,
    pub drink: String,
}

#[derive(AsChangeset, Debug, GraphQLInputObject, Insertable)]
#[table_name = "event_users"]
pub struct EventUserI {
    pub event_id: i32,
    pub user_id: i32,
    pub attends: bool,
}

#[derive(AsChangeset, Debug, GraphQLInputObject, Insertable)]
#[table_name = "user_rolls"]
pub struct UserRollI {
    pub event_user_id: i32,
    pub callback_query_id: String,
    pub jumbo: bool,
    pub alcoholic: bool,
    pub roll: i32,
    pub drink: String,
}

#[derive(Associations, Debug, Identifiable, Queryable)]
#[belongs_to(Chat)]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Message, foreign_key = "pinned_message_id")]
pub struct Message {
    pub id: i32,
    pub telegram_id: i32,
    pub chat_id: i32,
    pub user_id: Option<i32>,
    pub timestamp: NaiveDateTime,
    pub reply_to_message_id: Option<i32>,
    pub edit_timestamp: Option<NaiveDateTime>,
    pub text: Option<String>,
    pub caption: Option<String>,
    pub new_chat_member_ids: Option<Vec<i32>>,
    pub left_chat_member_id: Option<i32>,
    pub new_chat_title: Option<String>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub migrate_to_chat_id: Option<i32>,
    pub migrate_from_chat_id: Option<i32>,
    pub pinned_message_id: Option<i32>,
}

#[derive(AsChangeset, Debug, Insertable)]
#[table_name = "messages"]
pub struct MessageI {
    pub telegram_id: i32,
    pub chat_id: i32,
    pub user_id: Option<i32>,
    pub timestamp: NaiveDateTime,
    pub reply_to_message_id: Option<i32>,
    pub edit_timestamp: Option<NaiveDateTime>,
    pub text: Option<String>,
    pub caption: Option<String>,
    pub new_chat_member_ids: Option<Vec<i32>>,
    pub left_chat_member_id: Option<i32>,
    pub new_chat_title: Option<String>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub migrate_to_chat_id: Option<i32>,
    pub migrate_from_chat_id: Option<i32>,
    pub pinned_message_id: Option<i32>,
}

#[derive(AsChangeset, Associations, Debug, GraphQLObject, Identifiable, Insertable, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub telegram_id: i32,
    pub is_bot: bool,
    pub has_private_conversation: bool,
}

#[derive(AsChangeset, Debug, Insertable)]
#[table_name = "users"]
pub struct UserI {
    pub username: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub telegram_id: i32,
    pub is_bot: bool,
    pub has_private_conversation: bool,
}

#[derive(Debug, Identifiable, Queryable)]
pub struct Cocktail {
    pub id: i32,
    pub name: String,
    pub jumbo: bool,
    pub alcoholic: bool,
    pub category: i16,
}

#[derive(Debug, GraphQLObject, Identifiable, Queryable)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Identifiable, Queryable)]
pub struct CocktailIngredient {
    pub id: i32,
    pub cocktail_id: i32,
    pub ingredient_id: i32,
}

impl Chat {
    pub fn get_by_id(id: i32, connection: &diesel::PgConnection) -> DieselResult<Chat> {
        schema::chats::table
            .filter(schema::chats::dsl::id.eq(id))
            .first(connection)
    }

    pub fn get(limit: u32, connection: &diesel::PgConnection) -> DieselResult<Vec<Chat>> {
        schema::chats::table
            .limit(limit as i64)
            .load(connection)
    }
}

impl ChatI {
    pub fn insert(&self, connection: &diesel::PgConnection) -> DieselResult<Chat> {
        use schema::chats::dsl::*;

        diesel::insert_into(schema::chats::table)
            .values(self)
            .on_conflict(telegram_id)
            .do_update()
            .set(self)
            .load::<Chat>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}

impl ChatUserI {
    pub fn insert(&self, connection: &diesel::PgConnection) -> DieselResult<ChatUser> {
        use schema::chat_users::dsl::*;

        diesel::insert_into(schema::chat_users::table)
            .values(self)
            .on_conflict((user_id, chat_id))
            .do_update()
            .set(self)
            .load::<ChatUser>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}

impl Event {
    pub fn get_by_id(id: i32, connection: &diesel::PgConnection) -> DieselResult<Event> {
        schema::events::table
            .filter(schema::events::dsl::id.eq(id))
            .load::<Event>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }

    pub(crate) fn into_model_event(&self, connection: &diesel::PgConnection) -> DieselResult<models::Event> {
        let chat = Chat::get_by_id(self.chat_id, connection)?;

        Ok(models::Event {
            id: self.id,
            chat: chat.into(),
            timestamp: self.timestamp,
            active: self.active,
        })
    }
}

impl EventI {
    pub fn insert(&self, connection: &diesel::PgConnection) -> DieselResult<Event> {
        use schema::events::dsl::*;

        diesel::insert_into(schema::events::table)
            .values(self)
            .on_conflict((chat_id, timestamp))
            .do_update()
            .set(self)
            .execute(connection)
            .map_err(|_| diesel::NotFound)?;
        diesel::insert_into(schema::events::table)
            .values(self)
            .on_conflict((chat_id, active))
            .do_update()
            .set(self)
            .load::<Event>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}

impl EventUser {
    pub(crate) fn get_by_id(id: i32, connection: &diesel::PgConnection) -> DieselResult<EventUser> {
        schema::event_users::table
            .filter(schema::event_users::dsl::id.eq(id))
            .load::<EventUser>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}

impl EventUserI {
    pub fn insert(&self, connection: &diesel::PgConnection) -> DieselResult<EventUser> {
        use schema::event_users::dsl::*;

        diesel::insert_into(schema::event_users::table)
            .values(self)
            .on_conflict((event_id, user_id))
            .do_update()
            .set(self)
            .load::<EventUser>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}

impl MessageI {
    pub fn insert(&self, connection: &diesel::PgConnection) -> DieselResult<Message> {
        use schema::messages::dsl::*;

        diesel::insert_into(schema::messages::table)
            .values(self)
            .on_conflict((chat_id, telegram_id))
            .do_update()
            .set(self)
            .load::<Message>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}

impl UserI {
    pub fn insert(&self, connection: &diesel::PgConnection) -> DieselResult<User> {
        use schema::users::dsl::*;

        diesel::insert_into(schema::users::table)
            .values(self)
            .on_conflict(telegram_id)
            .do_update()
            .set(self)
            .load::<User>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}

impl UserRoll {
    pub(crate) fn get_by_id(id: i32, connection: &diesel::PgConnection) -> DieselResult<UserRoll> {
        schema::user_rolls::table
            .filter(schema::user_rolls::dsl::id.eq(id))
            .load::<UserRoll>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}

impl UserRollI {
    pub fn insert(&self, connection: &diesel::PgConnection) -> DieselResult<UserRoll> {
        use schema::user_rolls::dsl::*;

        diesel::insert_into(schema::user_rolls::table)
            .values(self)
            .on_conflict((event_user_id, callback_query_id))
            .do_update()
            .set(self)
            .load::<UserRoll>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}

impl Cocktail {
    pub(crate) fn get(connection: &diesel::PgConnection, constraints: Constraints) -> DieselResult<Vec<Cocktail>> {
        schema::cocktails::table
            .limit(constraints.limit as i64)
            .offset(constraints.offset as i64)
            .load::<Cocktail>(connection)
    }

    pub(crate) fn get_by_id(id: i32, connection: &diesel::PgConnection) -> DieselResult<Cocktail> {
        schema::cocktails::table
            .filter(schema::cocktails::dsl::id.eq(id))
            .load::<Cocktail>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}

impl Ingredient {
    pub(crate) fn get(connection: &diesel::PgConnection, constraints: Constraints) -> DieselResult<Vec<Ingredient>> {
        schema::ingredients::table
            .limit(constraints.limit as i64)
            .offset(constraints.offset as i64)
            .load::<Ingredient>(connection)
    }

    pub(crate) fn get_by_id(id: i32, connection: &diesel::PgConnection) -> DieselResult<Ingredient> {
        schema::ingredients::table
            .filter(schema::ingredients::dsl::id.eq(id))
            .load::<Ingredient>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}


impl CocktailIngredient {
    pub(crate) fn get(connection: &diesel::PgConnection, constraints: Constraints) -> DieselResult<Vec<CocktailIngredient>> {
        schema::cocktail_ingredients::table
            .limit(constraints.limit as i64)
            .offset(constraints.offset as i64)
            .load::<CocktailIngredient>(connection)
    }

    pub(crate) fn get_by_id(id: i32, connection: &diesel::PgConnection) -> DieselResult<CocktailIngredient> {
        schema::cocktail_ingredients::table
            .filter(schema::cocktail_ingredients::dsl::id.eq(id))
            .load::<CocktailIngredient>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}
