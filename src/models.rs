use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::database::models as db_models;
use crate::database::models::Ingredient;
use crate::database::schema;
use crate::graphql::schema::{ChatQuery, Constraints};

type DieselResult<T> = Result<T, diesel::result::Error>;

#[derive(Debug, GraphQLEnum)]
pub(crate) enum CocktailCategory {
    CAIPI = 0,
    JUMBO,
    VODKA,
    COLADAS,
    RUM,
    TEQUILA,
    FROZEN_MARGARITAS,
    GIN,
}

impl From<i16> for CocktailCategory {
    fn from(input: i16) -> Self {
        match input {
            0 => CocktailCategory::CAIPI,
            1 => CocktailCategory::JUMBO,
            2 => CocktailCategory::VODKA,
            3 => CocktailCategory::COLADAS,
            4 => CocktailCategory::RUM,
            5 => CocktailCategory::TEQUILA,
            6 => CocktailCategory::FROZEN_MARGARITAS,
            7 => CocktailCategory::GIN,
            _ => unimplemented!(),
        }
    }
}

impl ToString for CocktailCategory {
    fn to_string(&self) -> String {
        match self {
            CocktailCategory::CAIPI => "Caipi",
            CocktailCategory::JUMBO => "Jumbo",
            CocktailCategory::VODKA => "Vodka",
            CocktailCategory::COLADAS => "Coladas",
            CocktailCategory::RUM => "Rum",
            CocktailCategory::TEQUILA => "Tequila",
            CocktailCategory::FROZEN_MARGARITAS => "Frozen Margaritas",
            CocktailCategory::GIN => "Gin",
        }.to_string()
    }
}

#[derive(Debug, GraphQLEnum)]
pub enum ChatType {
    PRIVATE = 0,
    GROUP,
    SUPERGROUP,
}

impl From<i16> for ChatType {
    fn from(input: i16) -> ChatType {
        match input {
            0 => ChatType::PRIVATE,
            1 => ChatType::GROUP,
            2 => ChatType::SUPERGROUP,
            _ => unimplemented!()
        }
    }
}

#[derive(Debug, GraphQLEnum)]
pub enum KeyboardType {
    NONE = 0,
    ATTEND,
    DICE,
}

#[derive(Debug, GraphQLObject)]
pub(crate) struct Cocktail {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) jumbo: bool,
    pub(crate) alcoholic: bool,
    pub(crate) category: String,
    pub(crate) ingredients: Vec<db_models::Ingredient>,
}

#[derive(Debug, GraphQLObject)]
pub struct CocktailIngredient {
    pub(crate) id: i32,
    pub(crate) cocktail: Cocktail,
    pub(crate) ingredient: db_models::Ingredient,
}

impl From<i16> for KeyboardType {
    fn from(input: i16) -> KeyboardType {
        match input {
            0 => KeyboardType::NONE,
            1 => KeyboardType::ATTEND,
            2 => KeyboardType::DICE,
            _ => unimplemented!()
        }
    }
}

#[derive(Debug, GraphQLObject)]
pub struct Chat {
    pub id: i32,
    pub chat_type: ChatType,
    pub telegram_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub current_keyboard: KeyboardType,
    pub spam_detection: bool,
}

impl From<Chat> for db_models::Chat {
    fn from(input: Chat) -> Self {
        db_models::Chat {
            id: input.id,
            chat_type: input.chat_type as i16,
            telegram_id: input.telegram_id,
            title: input.title.clone(),
            description: input.description,
            current_keyboard: input.current_keyboard as i16,
            spam_detection: false,
        }
    }
}

impl From<Event> for db_models::Event {
    fn from(input: Event) -> Self {
        db_models::Event {
            id: input.id,
            chat_id: input.chat.id,
            timestamp: input.timestamp,
            active: input.active,
        }
    }
}

#[derive(Debug, GraphQLObject)]
pub struct ChatUser {
    pub(crate) id: i32,
    pub(crate) user: User,
    pub(crate) chat: Chat,
    pub(crate) spamming: bool,
    pub(crate) muted: bool,
    pub(crate) admin: bool,
}

#[derive(Debug, GraphQLObject)]
pub struct Event {
    pub id: i32,
    pub chat: Chat,
    pub timestamp: NaiveDateTime,
    pub active: bool,
}

#[derive(Debug, GraphQLObject)]
pub struct EventUser {
    pub(crate) id: i32,
    pub(crate) event: Event,
    pub(crate) user: User,
    pub(crate) attends: bool,
}

#[derive(Debug, GraphQLObject)]
pub struct UserRoll {
    pub(crate) id: i32,
    pub(crate) event_user: EventUser,
    pub(crate) jumbo: bool,
    pub(crate) alcoholic: bool,
    pub(crate) roll: i32,
    pub(crate) drink: String,
}

#[derive(Debug)]
pub struct Message {
    id: i32,
    telegram_id: i32,
    chat: Chat,
    user: Option<User>,
    timestamp: NaiveDateTime,
    //    reply_to: Option<Rc<RefCell<Message>>>,
    reply_to_message_id: Option<i32>,
    edit_timestamp: Option<NaiveDateTime>,
    text: Option<String>,
    caption: Option<String>,
    new_chat_members: Vec<User>,
    left_chat_member: Option<User>,
    new_chat_title: Option<String>,
    group_chat_created: Option<bool>,
    supergroup_chat_created: Option<bool>,
    migrate_to_chat: Option<Chat>,
    migrate_from_chat: Option<Chat>,
    //    pinned_message: Option<Rc<RefCell<Message>>>,
    pinned_message_id: Option<i32>,
}

pub type User = db_models::User;

impl Chat {
    pub(crate) fn get_by_query(connection: &diesel::PgConnection, constraints: Constraints, query: ChatQuery) -> DieselResult<Vec<Chat>> {
        use schema::chats::dsl::*;
        let mut sqlquery = schema::chats::table
            .limit(constraints.limit as i64)
            .offset(constraints.offset as i64)
            .into_boxed();

        if let Some(val) = query.title {
            sqlquery = sqlquery.or_filter(title.eq(val));
        }
        if let Some(val) = query.description {
            sqlquery = sqlquery.or_filter(description.eq(val));
        }
        if let Some(val) = query.id {
            sqlquery = sqlquery.or_filter(id.eq(val));
        }
        if let Some(val) = query.chat_type {
            let val = val as i16;
            sqlquery = sqlquery.or_filter(chat_type.eq(val));
        }
        if let Some(val) = query.telegram_id {
            sqlquery = sqlquery.or_filter(telegram_id.eq(val));
        }
        if let Some(val) = query.current_keyboard {
            let val = val as i16;
            sqlquery = sqlquery.or_filter(current_keyboard.eq(val));
        }
        if let Some(val) = query.spam_detection {
            sqlquery = sqlquery.or_filter(spam_detection.eq(val));
        }

        Ok(sqlquery
            .load::<db_models::Chat>(connection)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    pub fn get(connection: &diesel::PgConnection, constraints: Constraints) -> DieselResult<Vec<Chat>> {
        Ok(schema::chats::table
            .limit(constraints.limit as i64)
            .offset(constraints.offset as i64)
            .load::<db_models::Chat>(connection)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    pub fn get_by_id(id: i32, connection: &diesel::PgConnection) -> DieselResult<Chat> {
        Ok(schema::chats::table
            .find(id)
            .first::<db_models::Chat>(connection)?
            .into())
    }
}

impl Into<db_models::ChatI> for Chat {
    fn into(self) -> db_models::ChatI {
        db_models::ChatI {
            chat_type: self.chat_type as i16,
            telegram_id: self.telegram_id,
            title: self.title,
            description: self.description,
            current_keyboard: self.current_keyboard as i16,
            spam_detection: self.spam_detection,
        }
    }
}

impl Into<Chat> for db_models::Chat {
    fn into(self: db_models::Chat) -> Chat {
        Chat {
            id: self.id,
            chat_type: ChatType::from(self.chat_type as i16),
            telegram_id: self.telegram_id,
            title: self.title,
            description: self.description,
            current_keyboard: KeyboardType::from(self.current_keyboard),
            spam_detection: self.spam_detection,
        }
    }
}

impl ChatUser {
    pub fn from_database_model(chat_user: db_models::ChatUser, chat: db_models::Chat, user: db_models::User) -> ChatUser {
        ChatUser {
            id: chat_user.id,
            user: user.into(),
            chat: chat.into(),
            spamming: chat_user.spamming,
            muted: chat_user.muted,
            admin: chat_user.admin,
        }
    }

    fn from_loaded_vec(v: Vec<(db_models::ChatUser, db_models::Chat, db_models::User)>) -> Vec<ChatUser> {
        v
            .into_iter()
            .map(|(chat_user, chat, user): (db_models::ChatUser, db_models::Chat, db_models::User)| {
                ChatUser::from_database_model(chat_user, chat, user)
            })
            .collect()
    }

    pub fn full(connection: &diesel::PgConnection) -> DieselResult<Vec<ChatUser>> {
        Ok(
            ChatUser::from_loaded_vec(schema::chat_users::table
                .inner_join(schema::chats::table)
                .inner_join(schema::users::table)
                .load(connection)?
            ))
    }

    pub fn get_by_chat_id(chat_id: i32, connection: &diesel::PgConnection) -> DieselResult<Vec<ChatUser>> {
        Ok(
            ChatUser::from_loaded_vec(schema::chat_users::table
                .filter(schema::chat_users::dsl::chat_id.eq(chat_id))
                .inner_join(schema::chats::table)
                .inner_join(schema::users::table)
                .load(connection)?
            ))
    }

    pub fn get_by_user_id(user_id: i32, connection: &diesel::PgConnection) -> DieselResult<Vec<ChatUser>> {
        Ok(
            ChatUser::from_loaded_vec(schema::chat_users::table
                .filter(schema::chat_users::dsl::user_id.eq(user_id))
                .inner_join(schema::chats::table)
                .inner_join(schema::users::table)
                .load(connection)?))
    }
}

impl Into<db_models::ChatUserI> for ChatUser {
    fn into(self) -> db_models::ChatUserI {
        db_models::ChatUserI {
            user_id: self.user.id,
            chat_id: self.chat.id,
            spamming: self.spamming,
            muted: self.muted,
            admin: self.admin,
        }
    }
}

impl Event {
    fn from_database_model((event, chat): (db_models::Event, db_models::Chat)) -> Event {
        Event {
            id: event.id,
            chat: chat.into(),
            timestamp: event.timestamp,
            active: event.active,
        }
    }

    pub fn get(connection: &diesel::PgConnection, constraints: Constraints) -> DieselResult<Vec<Event>> {
        Ok(schema::events::table
            .inner_join(schema::chats::table)
            .limit(constraints.limit as i64)
            .offset(constraints.offset as i64)
            .load(connection)?
            .into_iter()
            .map(Event::from_database_model)
            .collect())
    }

    pub fn get_by_id(id: i32, connection: &diesel::PgConnection) -> DieselResult<Event> {
        schema::events::table
            .filter(schema::events::dsl::id.eq(id))
            .inner_join(schema::chats::table)
            .load(connection)?
            .pop()
            .map(Event::from_database_model)
            .ok_or(diesel::result::Error::NotFound)
    }
}

impl Into<db_models::EventI> for Event {
    fn into(self) -> db_models::EventI {
        db_models::EventI {
            chat_id: self.chat.id,
            timestamp: self.timestamp,
            active: self.active,
        }
    }
}

macro_rules! filter_eq {
        ($table:expr,$column:expr,$value:expr) => {
            $table
                .filter($column.eq($value))
    }}
impl EventUser {
    pub fn from_database_model(event_user: db_models::EventUser, event: db_models::Event, chat: db_models::Chat, user: db_models::User) -> EventUser {
        EventUser {
            id: event_user.id,
            event: Event {
                id: event.id,
                chat: chat.into(),
                timestamp: event.timestamp,
                active: event.active,
            },
            user: user.into(),
            attends: event_user.attends,
        }
    }

    pub fn get_by_chat(chat_id: i32, connection: &diesel::PgConnection) -> DieselResult<Vec<EventUser>> {
        Ok(schema::event_users::table
            .inner_join(schema::events::table
                .inner_join(schema::chats::table)
            )
            .inner_join(schema::users::table)
            .filter(schema::chats::id.eq(chat_id))
            .load(connection)?
            .into_iter()
            .map(|(event_user, (event, chat), user): (db_models::EventUser, (db_models::Event, db_models::Chat), db_models::User)|
                EventUser::from_database_model(event_user, event, chat, user)
            )
            .collect())
    }

    pub fn get_by_id(id: i32, connection: &diesel::PgConnection) -> DieselResult<EventUser> {
        let (event_user, (event, chat), user): (db_models::EventUser, (db_models::Event, db_models::Chat), db_models::User) =
            filter_eq!(schema::event_users::table, schema::event_users::id, id)
                .inner_join(schema::events::table
                    .inner_join(schema::chats::table))
                .inner_join(schema::users::table)
                .first(connection)?;

        Ok(EventUser::from_database_model(event_user, event, chat, user))
    }
}

impl Into<db_models::EventUserI> for EventUser {
    fn into(self) -> db_models::EventUserI {
        db_models::EventUserI {
            event_id: self.id,
            user_id: self.user.id,
            attends: self.attends,
        }
    }
}

impl Message {
    fn from_database_model(message: db_models::Message, chat: db_models::Chat, user: db_models::User, connection: &diesel::PgConnection) -> DieselResult<Message> {
        let left_chat_member = message
            .left_chat_member_id
            .and_then(|id|
                User::get_by_id(id, connection).ok()
            ).ok_or(diesel::NotFound)?;
        let new_chat_members = message
            .new_chat_member_ids
            .and_then(|ids|
                ids
                    .into_iter()
                    .map(|id| User::get_by_id(id, connection).ok())
                    .collect())
            .unwrap_or(vec![]);
        let migrate_to_chat = message
            .migrate_to_chat_id
            .and_then(|id|
                Chat::get_by_id(id, connection).ok());
        let migrate_from_chat = message
            .migrate_from_chat_id
            .and_then(|id|
                Chat::get_by_id(id, connection).ok());

        Ok(Message {
            id: message.id,
            telegram_id: message.telegram_id,
            chat: chat.into(),
            user: Some(user.into()),
            timestamp: message.timestamp,
            reply_to_message_id: message.reply_to_message_id,
            edit_timestamp: message.edit_timestamp,
            text: message.text,
            caption: message.caption,
            new_chat_members,
            left_chat_member: Some(left_chat_member),
            new_chat_title: message.new_chat_title,
            group_chat_created: message.group_chat_created,
            supergroup_chat_created: message.supergroup_chat_created,
            migrate_to_chat,
            migrate_from_chat,
            pinned_message_id: message.pinned_message_id,
        })
    }

    fn from_loaded_vec(v: Vec<(db_models::Message, db_models::Chat, db_models::User)>, connection: &diesel::PgConnection) -> DieselResult<Vec<Message>> {
        v
            .into_iter()
            .map(|(message, chat, user): (db_models::Message, db_models::Chat, db_models::User)| {
                Message::from_database_model(message, chat, user, connection)
            })
            .collect()
    }

    fn get_by_user_id(user_id: i32, connection: &diesel::PgConnection) -> DieselResult<Vec<Message>> {
        use schema::messages::dsl;

        Message::from_loaded_vec(dsl::messages
                                     .filter(dsl::user_id.eq(user_id))
                                     .inner_join(schema::chats::table)
                                     .inner_join(schema::users::table)
                                     .load::<(db_models::Message, db_models::Chat, db_models::User)>(connection)?,
                                 connection,
        )
    }

    fn get_by_chat_id(chat_id: i32, connection: &diesel::PgConnection) -> DieselResult<Vec<Message>> {
        use schema::messages::dsl;

        Message::from_loaded_vec(dsl::messages
                                     .filter(dsl::chat_id.eq(chat_id))
                                     .inner_join(schema::chats::table)
                                     .inner_join(schema::users::table)
                                     .load::<(db_models::Message, db_models::Chat, db_models::User)>(connection)?,
                                 connection)
    }
}

impl Into<db_models::MessageI> for Message {
    fn into(self) -> db_models::MessageI {
        db_models::MessageI {
            telegram_id: self.telegram_id,
            chat_id: self.chat.id,
            user_id: self.user.and_then(|user| Some(user.id)),
            timestamp: self.timestamp,
//            reply_to_message_id: self.reply_to.and_then(|message| Some(message.into_inner().id as i32)),
            reply_to_message_id: self.reply_to_message_id,
            edit_timestamp: self.edit_timestamp,
            text: self.text,
            caption: self.caption,
            new_chat_member_ids: self.new_chat_members.into_iter().map(|user| Some(user.id)).collect(),
            left_chat_member_id: self.left_chat_member.and_then(|user| Some(user.id)),
            new_chat_title: self.new_chat_title,
            group_chat_created: self.group_chat_created,
            supergroup_chat_created: self.supergroup_chat_created,
            migrate_to_chat_id: self.migrate_to_chat.and_then(|chat| Some(chat.id)),
            migrate_from_chat_id: self.migrate_from_chat.and_then(|chat| Some(chat.id)),
//            pinned_message_id: self.pinned_message.and_then(|message| Some(message.into_inner().id as i32)),
            pinned_message_id: self.pinned_message_id,
        }
    }
}

impl User {
    pub fn get(connection: &diesel::PgConnection, constraints: Constraints) -> DieselResult<Vec<User>> {
        schema::users::table
            .limit(constraints.limit as i64)
            .offset(constraints.offset as i64)
            .load::<db_models::User>(connection)
    }

    pub(crate) fn get_by_id(id: i32, connection: &diesel::PgConnection) -> DieselResult<User> {
        use schema::users::dsl;

        dsl::users
            .filter(dsl::id.eq(id))
            .first::<db_models::User>(connection)
    }

    fn get_by_telegram_id(telegram_id: i32, connection: &diesel::PgConnection) -> DieselResult<User> {
        use schema::users::dsl;

        dsl::users
            .filter(dsl::telegram_id.eq(telegram_id))
            .first::<db_models::User>(connection)
    }
}

impl Into<db_models::UserI> for User {
    fn into(self) -> db_models::UserI {
        db_models::UserI {
            username: self.username,
            first_name: self.first_name,
            last_name: self.last_name,
            telegram_id: self.telegram_id,
            is_bot: self.is_bot,
            has_private_conversation: self.has_private_conversation,
        }
    }
}

impl UserRoll {
    pub fn from_database_model(user_roll: db_models::UserRoll, event_user: EventUser) -> UserRoll {
        UserRoll {
            id: 0,
            event_user,
            jumbo: user_roll.jumbo,
            alcoholic: user_roll.alcoholic,
            roll: user_roll.roll,
            drink: user_roll.drink,
        }
    }

    pub fn get(connection: &diesel::PgConnection, constraints: Constraints) -> DieselResult<Vec<UserRoll>> {
        Ok(schema::user_rolls::table
            .inner_join(schema::event_users::table
                .inner_join(schema::events::table
                    .inner_join(schema::chats::table)
                )
                .inner_join(schema::users::table))
            .limit(constraints.limit as i64)
            .offset(constraints.offset as i64)
            .load(connection)?
            .into_iter()
            .map(|(user_roll, (event_user, (event, chat), user)): (db_models::UserRoll, (db_models::EventUser, (db_models::Event, db_models::Chat), db_models::User))| {
                let event_user = EventUser::from_database_model(event_user, event, chat, user);

                UserRoll::from_database_model(user_roll, event_user)
            }
            )
            .collect())
    }

    fn get_by_id(id: i32, connection: &diesel::PgConnection) -> DieselResult<UserRoll> {
        let dbmodel = db_models::UserRoll::get_by_id(id, connection)?;
        let event_user = EventUser::get_by_id(dbmodel.event_user_id, connection)?;

        Ok(UserRoll {
            id: dbmodel.id,
            event_user,
            jumbo: dbmodel.jumbo,
            alcoholic: dbmodel.alcoholic,
            roll: dbmodel.roll,
            drink: dbmodel.drink,
        })
    }
}

impl Cocktail {
    pub fn get(connection: &diesel::PgConnection, constraints: Constraints) -> DieselResult<Vec<Cocktail>> {
        Ok(db_models::Cocktail::get(connection, constraints)?
            .into_iter()
            .filter_map(|cocktail| {
                Some(Cocktail {
                    id: cocktail.id,
                    name: cocktail.name,
                    jumbo: cocktail.jumbo,
                    alcoholic: cocktail.alcoholic,
                    category: CocktailCategory::from(cocktail.category).to_string(),
                    ingredients: CocktailIngredient::get_by_cocktail(cocktail.id, connection, constraints).ok()?,
                })
            })
            .collect())
    }
}

impl CocktailIngredient {
    pub fn get_by_cocktail(cocktail_id: i32, connection: &diesel::PgConnection, constraints: Constraints) -> DieselResult<Vec<Ingredient>> {
        Ok(schema::cocktail_ingredients::table
            .inner_join(schema::cocktails::table)
            .inner_join(schema::ingredients::table)
            .limit(constraints.limit as i64)
            .offset(constraints.offset as i64)
            .filter(schema::cocktails::id.eq(cocktail_id))
            .load(connection)?
            .into_iter()
            .map(|(_, _, ingredient): (db_models::CocktailIngredient, db_models::Cocktail, db_models::Ingredient)| {
                ingredient
            })
            .collect())
    }
}
