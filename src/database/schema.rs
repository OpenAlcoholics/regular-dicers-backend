table! {
    chat_users (id) {
        id -> Int4,
        chat_id -> Int4,
        user_id -> Int4,
        spamming -> Bool,
        muted -> Bool,
        admin -> Bool,
    }
}

table! {
    chats (id) {
        id -> Int4,
        chat_type -> Int2,
        telegram_id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        current_keyboard -> Int2,
        spam_detection -> Bool,
    }
}

table! {
    cocktail_ingredients (id) {
        id -> Int4,
        cocktail_id -> Int4,
        ingredient_id -> Int4,
    }
}

table! {
    cocktails (id) {
        id -> Int4,
        name -> Varchar,
        jumbo -> Bool,
        alcoholic -> Bool,
        category -> Int2,
    }
}

table! {
    event_users (id) {
        id -> Int4,
        event_id -> Int4,
        user_id -> Int4,
        attends -> Bool,
    }
}

table! {
    events (id) {
        id -> Int4,
        chat_id -> Int4,
        timestamp -> Timestamp,
        active -> Bool,
    }
}

table! {
    ingredients (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    messages (id) {
        id -> Int4,
        telegram_id -> Int4,
        chat_id -> Int4,
        user_id -> Nullable<Int4>,
        timestamp -> Timestamp,
        reply_to_message_id -> Nullable<Int4>,
        edit_timestamp -> Nullable<Timestamp>,
        text -> Nullable<Text>,
        caption -> Nullable<Text>,
        new_chat_member_ids -> Nullable<Array<Int4>>,
        left_chat_member_id -> Nullable<Int4>,
        new_chat_title -> Nullable<Text>,
        group_chat_created -> Nullable<Bool>,
        supergroup_chat_created -> Nullable<Bool>,
        migrate_to_chat_id -> Nullable<Int4>,
        migrate_from_chat_id -> Nullable<Int4>,
        pinned_message_id -> Nullable<Int4>,
    }
}

table! {
    user_rolls (id) {
        id -> Int4,
        event_user_id -> Int4,
        callback_query_id -> Text,
        jumbo -> Bool,
        alcoholic -> Bool,
        roll -> Int4,
        drink -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Nullable<Text>,
        first_name -> Text,
        last_name -> Nullable<Text>,
        telegram_id -> Int4,
        is_bot -> Bool,
        has_private_conversation -> Bool,
    }
}

joinable!(chat_users -> chats (chat_id));
joinable!(chat_users -> users (user_id));
joinable!(cocktail_ingredients -> cocktails (cocktail_id));
joinable!(cocktail_ingredients -> ingredients (ingredient_id));
joinable!(event_users -> events (event_id));
joinable!(event_users -> users (user_id));
joinable!(events -> chats (chat_id));
joinable!(messages -> chats (chat_id));
joinable!(messages -> users (user_id));
joinable!(user_rolls -> event_users (event_user_id));

allow_tables_to_appear_in_same_query!(
    chat_users,
    chats,
    cocktail_ingredients,
    cocktails,
    event_users,
    events,
    ingredients,
    messages,
    user_rolls,
    users,
);
