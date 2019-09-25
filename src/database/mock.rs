use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use chrono::NaiveDateTime;

use crate::database::models::*;

pub trait Mockable {
    type Item;

    fn mock(extra_data: Option<HashMap<String, i32>>) -> Option<Self::Item>;
}

fn wrap_random<T: Sized, F>(random: F) -> Option<T>
    where F: Fn() -> T {
    match rand::random::<u8>() % 2 {
        0 => Some(random()),
        _ => None
    }
}

fn random_i32() -> i32 {
    rand::random()
}

fn random_string(n: u8) -> String {
    (0..n)
        // Only create values in range 33 - 126 (non formatting characters)
        .map(|_: u8| ((rand::random::<u8>() % (126 - 33)) + 33) as u8)
        .map(|u| char::from(u))
        .collect::<String>()
        .to_ascii_lowercase()
}

fn random_bool() -> bool {
    rand::random()
}

fn random_optional_string(n: u8) -> Option<String> {
    match rand::random::<u8>() % 2 {
        0 => Some(random_string(n)),
        _ => None
    }
}

fn random_roll() -> u8 {
    (rand::random::<u8>() % 6) + 1
}

fn random_chat_type() -> u8 {
    rand::random::<u8>() % 3
}

fn random_keyboard_type() -> u8 {
    rand::random::<u8>() % 3
}

fn random_date_time() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(rand::random::<u32>() as i64, 0)
}

impl Mockable for ChatI {
    type Item = ChatI;

    fn mock(_: Option<HashMap<String, i32, RandomState>>) -> Option<Self::Item> {
        Some(ChatI {
            chat_type: random_chat_type() as i16,
            telegram_id: random_i32(),
            title: random_string(16),
            description: random_optional_string(64),
            current_keyboard: random_keyboard_type() as i16,
            spam_detection: random_bool(),
        })
    }
}

impl Mockable for ChatUserI {
    type Item = ChatUserI;

    fn mock(extra_data: Option<HashMap<String, i32, RandomState>>) -> Option<Self::Item> {
        let data = extra_data?;
        let chat_id = data.get("chat_id")?;
        let user_id = data.get("user_id")?;

        let admin = random_bool();
        Some(ChatUserI {
            user_id: *user_id,
            chat_id: *chat_id,
            spamming: random_bool(),
            muted: match admin {
                true => false,
                _ => random_bool()
            }, // User can't be muted while being an admin
            admin,
        })
    }
}

impl Mockable for EventI {
    type Item = EventI;

    fn mock(extra_data: Option<HashMap<String, i32, RandomState>>) -> Option<Self::Item> {
        Some(EventI {
            chat_id: *extra_data?.get("chat_id")?,
            timestamp: random_date_time(),
            active: random_bool(),
        })
    }
}

impl Mockable for EventUserI {
    type Item = EventUserI;

    fn mock(extra_data: Option<HashMap<String, i32, RandomState>>) -> Option<Self::Item> {
        let data = extra_data?;
        let event_id = data.get("event_id")?;
        let user_id = data.get("user_id")?;
        let attends = random_bool();

        Some(EventUserI {
            event_id: *event_id,
            user_id: *user_id,
            attends,
        })
    }
}

impl Mockable for MessageI {
    type Item = MessageI;

    fn mock(extra_data: Option<HashMap<String, i32, RandomState>>) -> Option<Self::Item> {
        let data = extra_data?;
        let chat_id = data.get("chat_id")?;
        let user_id = data.get("user_id");

        Some(MessageI {
            telegram_id: random_i32(),
            user_id: user_id.and_then(|x| Some(*x)),
            chat_id: *chat_id,
            timestamp: random_date_time(),
            reply_to_message_id: None,
            edit_timestamp: wrap_random(random_date_time),
            text: random_optional_string(128),
            caption: random_optional_string(16),
            new_chat_member_ids: None,
            left_chat_member_id: None,
            new_chat_title: random_optional_string(16),
            group_chat_created: wrap_random(random_bool),
            supergroup_chat_created: wrap_random(random_bool),
            migrate_to_chat_id: None,
            migrate_from_chat_id: None,
            pinned_message_id: None,
        })
    }
}

impl Mockable for UserI {
    type Item = UserI;

    fn mock(_: Option<HashMap<String, i32, RandomState>>) -> Option<Self::Item> {
        Some(UserI {
            username: random_optional_string(16),
            first_name: random_string(16),
            last_name: random_optional_string(16),
            telegram_id: random_i32(),
            is_bot: random_bool(),
            has_private_conversation: random_bool(),
        })
    }
}

impl Mockable for UserRollI {
    type Item = UserRollI;

    fn mock(extra_data: Option<HashMap<String, i32, RandomState>>) -> Option<Self::Item> {
        let data = extra_data?;
        let event_user_id = data.get("event_user_id")?;

        Some(UserRollI {
            event_user_id: *event_user_id,
            callback_query_id: random_string(16),
            jumbo: random_bool(),
            alcoholic: random_bool(),
            roll: random_roll() as i32,
            drink: random_string(16)
        })
    }
}
