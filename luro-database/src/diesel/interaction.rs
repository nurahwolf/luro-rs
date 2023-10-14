use diesel::{Queryable, Selectable};
use twilight_model::application::interaction::InteractionData;

#[derive(diesel_derive_enum::DbEnum, Debug)]
#[ExistingTypePath = "crate::schema::sql_types::InteractionKind"]
pub enum DatabaseInteractionKind {
    ApplicationCommand,
    ApplicationCommandAutocomplete,
    MessageComponent,
    ModalSubmit,
    Ping,
    Unknown,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::interactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DatabaseInteraction {
    pub app_permissions: Option<i64>,
    pub application_id: i64,
    pub channel_id: i64,
    pub data: Option<InteractionData>,
    pub guild_id: Option<i64>,
    pub guild_locale: Option<String>,
    pub interaction_id: i64,
    pub kind: DatabaseInteractionKind,
    pub locale: Option<String>,
    pub member_id: Option<i64>,
    pub message_id: Option<i64>,
    pub token: String,
    pub user_id: Option<i64>,
}