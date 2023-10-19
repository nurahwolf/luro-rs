// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "interaction_kind"))]
    pub struct InteractionKind;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "message_source"))]
    pub struct MessageSource;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_permissions"))]
    pub struct UserPermissions;
}

diesel::table! {
    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    channels (channel_id) {
        channel_id -> Int8,
    }
}

diesel::table! {
    guilds (guild_id) {
        name -> Text,
        guild_id -> Int8,
        owner_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::InteractionKind;

    interactions (interaction_id) {
        app_permissions -> Nullable<Int8>,
        application_id -> Int8,
        channel_id -> Int8,
        data -> Nullable<Jsonb>,
        guild_id -> Nullable<Int8>,
        guild_locale -> Nullable<Text>,
        interaction_id -> Int8,
        kind -> InteractionKind,
        locale -> Nullable<Text>,
        member_id -> Nullable<Int8>,
        message_id -> Nullable<Int8>,
        token -> Text,
        user_id -> Nullable<Int8>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MessageSource;

    messages (id) {
        activity -> Nullable<Jsonb>,
        application -> Nullable<Jsonb>,
        application_id -> Nullable<Int8>,
        attachments -> Nullable<Jsonb>,
        author -> Jsonb,
        channel_id -> Int8,
        components -> Nullable<Jsonb>,
        content -> Nullable<Text>,
        deleted -> Nullable<Bool>,
        edited_timestamp -> Nullable<Timestamptz>,
        embeds -> Nullable<Jsonb>,
        flags -> Nullable<Jsonb>,
        guild_id -> Nullable<Int8>,
        id -> Int8,
        interaction -> Nullable<Jsonb>,
        kind -> Jsonb,
        member -> Nullable<Jsonb>,
        mention_channels -> Nullable<Jsonb>,
        mention_everyone -> Nullable<Bool>,
        mention_roles -> Nullable<Array<Nullable<Int8>>>,
        mentions -> Nullable<Jsonb>,
        message_updates -> Nullable<Jsonb>,
        pinned -> Nullable<Bool>,
        reactions -> Nullable<Jsonb>,
        reference -> Nullable<Jsonb>,
        referenced_message -> Nullable<Jsonb>,
        role_subscription_data -> Nullable<Jsonb>,
        source -> MessageSource,
        sticker_items -> Nullable<Jsonb>,
        thread -> Nullable<Jsonb>,
        timestamp -> Timestamptz,
        tts -> Nullable<Bool>,
        webhook_id -> Nullable<Int8>,
    }
}

diesel::table! {
    roles (role_id) {
        colour -> Int4,
        deleted -> Bool,
        flags -> Int8,
        hoist -> Bool,
        icon -> Nullable<Jsonb>,
        managed -> Bool,
        mentionable -> Bool,
        name -> Text,
        permissions -> Int8,
        position -> Int8,
        role_id -> Int8,
        tags -> Nullable<Jsonb>,
        unicode_emoji -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserPermissions;

    users (user_id) {
        accent_colour -> Nullable<Int4>,
        avatar -> Nullable<Jsonb>,
        avatar_decoration -> Nullable<Jsonb>,
        averagesize -> Nullable<Int8>,
        banner -> Nullable<Jsonb>,
        bot -> Nullable<Bool>,
        characters -> Nullable<Array<Nullable<Int4>>>,
        discriminator -> Int2,
        email -> Nullable<Text>,
        flags -> Nullable<Jsonb>,
        global_name -> Nullable<Text>,
        locale -> Nullable<Text>,
        message_edits -> Nullable<Int8>,
        messages -> Nullable<Array<Nullable<Int8>>>,
        mfa_enabled -> Nullable<Bool>,
        moderation_actions -> Nullable<Jsonb>,
        moderation_actions_performed -> Nullable<Int8>,
        name -> Text,
        premium_type -> Nullable<Jsonb>,
        public_flags -> Nullable<Jsonb>,
        system -> Nullable<Bool>,
        user_id -> Int8,
        verified -> Nullable<Bool>,
        warnings -> Nullable<Array<Nullable<Int8>>>,
        words_average -> Nullable<Int8>,
        words_count -> Nullable<Int8>,
        user_permissions -> Nullable<UserPermissions>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(_sqlx_migrations, channels, guilds, interactions, messages, roles, users,);
