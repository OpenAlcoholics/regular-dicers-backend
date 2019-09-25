use std::convert::*;

use chrono::NaiveDateTime;
use juniper::FieldResult;

use crate::database::models as dbmodels;
use crate::graphql::{Context, MutationRoot, QueryRoot};
use crate::models;
use crate::models::{ChatUser, Cocktail, Event, EventUser, UserRoll};

#[derive(GraphQLInputObject)]
struct InputChat {
    pub chat_type: ChatType,
    pub telegram_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub current_keyboard: KeyboardType,
    pub spam_detection: bool,
}

impl From<InputChat> for dbmodels::ChatI {
    fn from(input: InputChat) -> Self {
        dbmodels::ChatI {
            chat_type: input.chat_type as i16,
            telegram_id: input.telegram_id,
            title: input.title.clone(),
            description: input.description,
            current_keyboard: input.current_keyboard as i16,
            spam_detection: input.spam_detection,
        }
    }
}

impl From<InputEvent> for dbmodels::EventI {
    fn from(input: InputEvent) -> Self {
        dbmodels::EventI {
            chat_id: input.chat_id,
            timestamp: input.timestamp,
            active: input.active,
        }
    }
}

#[derive(GraphQLInputObject)]
struct InputUser {
    username: Option<String>,
    first_name: String,
    last_name: Option<String>,
    telegram_id: i32,
    is_bot: bool,
    has_private_conversation: bool,
}

impl From<InputUser> for dbmodels::UserI {
    fn from(input: InputUser) -> Self {
        dbmodels::UserI {
            username: input.username,
            first_name: input.first_name.clone(),
            last_name: input.last_name,
            telegram_id: input.telegram_id,
            is_bot: input.is_bot,
            has_private_conversation: input.has_private_conversation,
        }
    }
}

#[derive(Clone, Copy, GraphQLInputObject)]
pub struct Constraints {
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for Constraints {
    fn default() -> Self {
        Constraints {
            limit: 100,
            offset: 0,
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct User {
    pub id: i32,
    pub username: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub telegram_id: i32,
    pub is_bot: bool,
}

#[derive(Debug, juniper::GraphQLEnum)]
pub enum ChatType {
    PRIVATE = 0,
    GROUP,
    SUPERGROUP,
}

impl From<models::ChatType> for ChatType {
    fn from(input: models::ChatType) -> Self {
        match input {
            models::ChatType::PRIVATE => ChatType::PRIVATE,
            models::ChatType::GROUP => ChatType::GROUP,
            models::ChatType::SUPERGROUP => ChatType::SUPERGROUP,
        }
    }
}

#[derive(Debug, juniper::GraphQLEnum)]
pub enum KeyboardType {
    NONE = 0,
    ATTEND,
    DICE,
}

impl From<models::KeyboardType> for KeyboardType {
    fn from(input: models::KeyboardType) -> Self {
        match input {
            models::KeyboardType::NONE => KeyboardType::NONE,
            models::KeyboardType::ATTEND => KeyboardType::ATTEND,
            models::KeyboardType::DICE => KeyboardType::DICE,
        }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct Chat {
    id: i32,
    chat_type: ChatType,
    telegram_id: i32,
    title: String,
    description: Option<String>,
    current_keyboard: KeyboardType,
    spam_detection: bool,
}

impl From<models::Chat> for Chat {
    fn from(input: models::Chat) -> Self {
        Chat {
            id: input.id,
            chat_type: input.chat_type.into(),
            telegram_id: input.telegram_id,
            title: input.title.clone(),
            description: input.description,
            current_keyboard: input.current_keyboard.into(),
            spam_detection: input.spam_detection,
        }
    }
}

impl From<models::User> for User {
    fn from(input: models::User) -> Self {
        User {
            id: input.id,
            username: input.username,
            first_name: input.first_name.clone(),
            last_name: input.last_name,
            telegram_id: input.telegram_id,
            is_bot: input.is_bot,
        }
    }
}

#[derive(GraphQLInputObject)]
pub struct InputEvent {
    chat_id: i32,
    timestamp: NaiveDateTime,
    active: bool,
}

type InputUserRoll = dbmodels::UserRollI;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn user(&self, context: &Context, user: InputUser) -> FieldResult<User> {
        Ok(User::from(
            models::User::from(
                dbmodels::UserI::from(user)
                    .insert(&context.connection.0)?
            )
        ))
    }

    fn user_roll(&self, context: &Context, roll: InputUserRoll) -> FieldResult<UserRoll> {
        let event_user_id = roll.event_user_id;

        let connection = &context.connection.0;
        let dbmodel: dbmodels::UserRoll = dbmodels::UserRollI::from(roll)
            .insert(connection)?;
        let event_user = models::EventUser::get_by_id(event_user_id, connection)?;

        Ok(UserRoll {
            id: dbmodel.id,
            event_user,
            jumbo: dbmodel.jumbo,
            alcoholic: dbmodel.alcoholic,
            roll: dbmodel.roll,
            drink: dbmodel.drink,
        })
    }

    fn chat(&self, context: &Context, chat: InputChat) -> FieldResult<Chat> {
        let dbmodel: dbmodels::Chat = dbmodels::ChatI::from(chat)
            .insert(&context.connection.0)?;
        let model: models::Chat = dbmodel.into();

        Ok(model.into())
    }

    fn chat_user(&self, context: &Context, chat_user: dbmodels::ChatUserI) -> FieldResult<ChatUser> {
        let connection = &context.connection.0;
        let dbmodel = chat_user.insert(connection)?;
        let chat: models::Chat = dbmodels::Chat::get_by_id(dbmodel.chat_id, connection)?.into();
        let user: models::User = dbmodels::User::get_by_id(dbmodel.user_id, connection)?.into();

        Ok(ChatUser {
            id: dbmodel.id,
            user,
            chat,
            spamming: dbmodel.spamming,
            muted: dbmodel.muted,
            admin: dbmodel.admin,
        })
    }

    fn event_user(&self, context: &Context, event_user: dbmodels::EventUserI) -> FieldResult<EventUser> {
        let connection = &context.connection.0;
        let dbmodel = event_user.insert(connection)?;
        let event = dbmodels::Event::get_by_id(dbmodel.event_id, connection)?.into_model_event(connection)?;
        let user = dbmodels::User::get_by_id(dbmodel.user_id, connection)?;

        Ok(EventUser {
            id: dbmodel.id,
            event,
            user,
            attends: dbmodel.attends,
        })
    }

    fn event(&self, context: &Context, event: InputEvent) -> FieldResult<Event> {
        let connection = &context.connection.0;
        let dbmodel = dbmodels::EventI::from(event).insert(connection)?;
        let chat: dbmodels::Chat = dbmodels::Chat::get_by_id(dbmodel.chat_id, connection)?;

        let model: models::Event = dbmodel.into_model_event(connection)?;

        Ok(model.into())
    }
}

#[derive(GraphQLInputObject)]
pub(crate) struct ChatQuery {
    pub(crate) id: Option<i32>,
    pub(crate) chat_type: Option<ChatType>,
    pub(crate) telegram_id: Option<i32>,
    pub(crate) title: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) current_keyboard: Option<KeyboardType>,
    pub(crate) spam_detection: Option<bool>,
}

#[juniper::object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "All fields inside `query` are joined by an OR")]
    fn chats(&self, context: &Context, constraints: Option<Constraints>, query: Option<ChatQuery>) -> FieldResult<Vec<Chat>> {
        let constraints = constraints.unwrap_or_default();

        // performance optimization, if `query` is undefined, we can just get all chats (limited by `constraints`)
        Ok(match query {
            Some(val) => models::Chat::get_by_query(&context.connection.0, constraints, val),
            None => models::Chat::get(&context.connection.0, constraints)
        }?.into_iter()
            .map(Into::into)
            .collect())
    }

    fn events(&self, context: &Context, constraints: Option<Constraints>) -> FieldResult<Vec<models::Event>> {
        let constraints = constraints.unwrap_or_default();

        Ok(models::Event::get(&context.connection.0, constraints)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    fn users(&self, context: &Context, constraints: Option<Constraints>) -> FieldResult<Vec<User>> {
        let constraints = constraints.unwrap_or_default();

        Ok(models::User::get(&context.connection.0, constraints)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    fn user_rolls(&self, context: &Context, constraints: Option<Constraints>) -> FieldResult<Vec<UserRoll>> {
        let constraints = constraints.unwrap_or_default();

        Ok(models::UserRoll::get(&context.connection.0, constraints)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    fn cocktails(&self, context: &Context, constraints: Option<Constraints>) -> FieldResult<Vec<Cocktail>> {
        let constraints = constraints.unwrap_or_default();

        Ok(Cocktail::get(&context.connection.0, constraints)?)
    }
}
