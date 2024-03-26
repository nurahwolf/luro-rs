use sqlx::types::Json;
use twilight_model::application::interaction::{Interaction, InteractionType as TwiInteractionType};

use crate::database::sqlx::Error;

impl crate::database::sqlx::Database {
    pub async fn update_interaction(&self, interaction: &Interaction) -> Result<u64, Error> {
        let mut rows_updated = 0;
        if let Some(channel) = &interaction.channel {
            match self.update_channel(channel).await {
                Ok(rows) => rows_updated += rows,
                Err(why) => tracing::warn!(why = ?why, "INTERACTION - Failed to sync channel {}", channel.id),
            }
        }

        if let Some(user) = &interaction.user {
            match self.update_user(user).await {
                Ok(rows) => rows_updated += rows,
                Err(why) => tracing::warn!(why = ?why, "INTERACTION - Failed to sync user {}", user.id),
            }
        }

        if let Some(guild_id) = interaction.guild_id {
            if let Some(member) = &interaction.member {
                match self.update_user((guild_id, member)).await {
                    Ok(rows) => rows_updated += rows,
                    Err(why) => {
                        let name = member.user.as_ref().map(|x| x.name.as_ref()).unwrap_or("UNKNOWN");
                        tracing::warn!(why = ?why, "INTERACTION - Failed to sync member {name}");
                    }
                }
            }
        }

        rows_updated += sqlx::query_file!(
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
        .await?
        .rows_affected();

        Ok(rows_updated)
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
