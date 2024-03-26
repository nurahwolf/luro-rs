use sqlx::types::Json;
use twilight_model::application::interaction::{Interaction, InteractionType as TwiInteractionType};

use crate::database::sqlx::Error;

impl crate::database::sqlx::Database {
    pub async fn update_interaction(&self, interaction: &Interaction) -> Result<u64, Error> {
        Ok(sqlx::query_file!(
            "queries/interaction/interaction_update.sql",
            interaction.app_permissions.map(|x| x.bits() as i64),
            interaction.application_id.get() as i64,
            interaction.channel.as_ref().map(|x| x.id.get() as i64),
            interaction.data.as_ref().map(Json) as _,
            interaction.guild_id.map(|x| x.get() as i64),
            interaction.guild_locale,
            interaction.id.get() as i64,
            match interaction.kind {
                TwiInteractionType::Ping => InteractionType::Ping,
                TwiInteractionType::ApplicationCommand => InteractionType::ApplicationCommand,
                TwiInteractionType::MessageComponent => InteractionType::MessageComponent,
                TwiInteractionType::ApplicationCommandAutocomplete => InteractionType::ApplicationCommandAutocomplete,
                TwiInteractionType::ModalSubmit => InteractionType::ModalSubmit,
                _ => InteractionType::Unknown,
            } as _,
            interaction.locale,
            interaction.message.as_ref().map(|x| x.id.get() as i64),
            interaction.token,
            interaction.author_id().unwrap().get() as i64,
        )
        .execute(&self.pool)
        .await
        .map(|x| x.rows_affected())?)
    }
}

#[derive(Debug, ::sqlx::Type)]
#[sqlx(type_name = "interaction_kind", rename_all = "SCREAMING_SNAKE_CASE")]
enum InteractionType {
    ApplicationCommand,
    ApplicationCommandAutocomplete,
    MessageComponent,
    ModalSubmit,
    Ping,
    Unknown,
}
