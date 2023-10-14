use sqlx::types::Json;
use twilight_model::application::interaction::{Interaction, InteractionData};

mod count_interactions;
mod get_interaction;
mod update_interaction;

#[derive(Debug, ::sqlx::Type)]
#[sqlx(type_name = "interaction_kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DatabaseInteractionKind {
    ApplicationCommand,
    ApplicationCommandAutocomplete,
    MessageComponent,
    ModalSubmit,
    Ping,
    Unknown,
}

#[derive(Debug)]
pub struct DatabaseInteraction {
    pub app_permissions: Option<i64>,
    pub application_id: i64,
    pub channel_id: i64,
    pub data: Option<Json<InteractionData>>,
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

impl From<Interaction> for DatabaseInteraction {
    fn from(interaction: Interaction) -> Self {
        Self {
            app_permissions: interaction.app_permissions.map(|x| x.bits() as i64),
            application_id: interaction.application_id.get() as i64,
            channel_id: interaction.channel.map(|x| x.id.get() as i64).unwrap_or_default(),
            data: interaction.data.map(Json),
            guild_id: interaction.guild_id.map(|x| x.get() as i64),
            guild_locale: interaction.guild_locale,
            interaction_id: interaction.id.get() as i64,
            kind: match interaction.kind {
                twilight_model::application::interaction::InteractionType::ApplicationCommand => {
                    DatabaseInteractionKind::ApplicationCommand
                }
                twilight_model::application::interaction::InteractionType::ApplicationCommandAutocomplete => {
                    DatabaseInteractionKind::ApplicationCommandAutocomplete
                }
                twilight_model::application::interaction::InteractionType::MessageComponent => DatabaseInteractionKind::MessageComponent,
                twilight_model::application::interaction::InteractionType::ModalSubmit => DatabaseInteractionKind::ModalSubmit,
                twilight_model::application::interaction::InteractionType::Ping => DatabaseInteractionKind::Ping,
                _ => DatabaseInteractionKind::Unknown,
            },
            locale: interaction.locale,
            member_id: interaction.member.map(|x| x.user.map(|x| x.id.get() as i64)).unwrap_or_default(),
            message_id: interaction.message.map(|x| x.id.get() as i64),
            token: interaction.token,
            user_id: interaction.user.map(|x| x.id.get() as i64),
        }
    }
}
