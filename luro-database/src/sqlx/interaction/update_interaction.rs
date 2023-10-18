use sqlx::types::Json;
use twilight_model::application::interaction::InteractionData;

use crate::{DatabaseInteraction, DatabaseInteractionKind, LuroDatabase};
impl LuroDatabase {
    pub async fn update_interaction(&self, interaction: DatabaseInteraction) -> anyhow::Result<DatabaseInteraction> {
        let query = sqlx::query_as!(
            DatabaseInteraction,
            "INSERT INTO interactions (
                app_permissions,
                application_id,
                channel_id,
                data,
                guild_id,
                guild_locale,
                interaction_id,
                kind,
                locale,
                message_id,
                token,
                user_id
            ) VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT
                (interaction_id)
            DO UPDATE SET
                app_permissions = $1,
                application_id = $2,
                channel_id = $3,
                data = $4,
                guild_id = $5,
                guild_locale = $6,
                interaction_id = $7,
                kind = $8,
                locale = $9,
                message_id = $10,
                user_id = $12
            RETURNING 
                app_permissions,
                application_id,
                channel_id,
                data as \"data: Json<InteractionData>\",
                guild_id,
                guild_locale,
                interaction_id,
                kind as \"kind: DatabaseInteractionKind\",
                locale,
                message_id,
                token,
                user_id
            ",
            interaction.app_permissions,
            interaction.application_id,
            interaction.channel_id,
            interaction.data as _,
            interaction.guild_id,
            interaction.guild_locale,
            interaction.interaction_id,
            interaction.kind as _,
            interaction.locale,
            interaction.message_id,
            interaction.token,
            interaction.user_id,
        );

        Ok(query.fetch_one(&self.pool).await?)
    }
}
