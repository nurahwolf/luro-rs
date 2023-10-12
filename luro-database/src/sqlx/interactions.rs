use sqlx::types::Json;
use twilight_model::application::interaction::Interaction;

use crate::{DatabaseInteraction, DatabaseInteractionKind};

mod count_interactions;
mod get_interaction;
mod update_interaction;

impl From<Interaction> for DatabaseInteraction {
    fn from(interaction: Interaction) -> Self {
        Self {
            app_permissions: interaction.app_permissions.map(|x| x.bits() as i64),
            application_id: interaction.application_id.get() as i64,
            channel_id: interaction.channel.map(|x| x.id.get() as i64),
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
                twilight_model::application::interaction::InteractionType::MessageComponent => {
                    DatabaseInteractionKind::MessageComponent
                }
                twilight_model::application::interaction::InteractionType::ModalSubmit => DatabaseInteractionKind::ModalSubmit,
                twilight_model::application::interaction::InteractionType::Ping => DatabaseInteractionKind::Ping,
                _ => DatabaseInteractionKind::Unknown,
            },
            locale: interaction.locale,
            member_id: interaction
                .member
                .map(|x| x.user.map(|x| x.id.get() as i64))
                .unwrap_or_default(),
            message_id: interaction.message.map(|x| x.id.get() as i64),
            token: interaction.token,
            user_id: interaction.user.map(|x| x.id.get() as i64),
        }
    }
}
