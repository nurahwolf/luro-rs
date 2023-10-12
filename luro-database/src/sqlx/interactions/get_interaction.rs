use sqlx::types::Json;
use twilight_model::application::interaction::InteractionData;

use crate::{DatabaseInteraction, DatabaseInteractionKind, LuroDatabase};

impl LuroDatabase {
    /// Fetches an interaction by message_id
    pub async fn get_interaction_by_message_id(&self, id: i64) -> Result<Option<DatabaseInteraction>, sqlx::Error> {
        let query = sqlx::query_as!(
            DatabaseInteraction,
            "SELECT 
                app_permissions,
                application_id,
                channel_id,
                data as \"data: Json<InteractionData>\",
                guild_id,
                guild_locale,
                interaction_id,
                kind as \"kind: DatabaseInteractionKind\",
                locale,
                member_id,
                message_id,
                token,
                user_id
            FROM interactions WHERE message_id = $1",
            id
        );

        query.fetch_optional(&self.0).await
    }

    /// Fetches an interaction by interaction_id
    pub async fn get_interaction(&self, id: i64) -> Result<Option<DatabaseInteraction>, sqlx::Error> {
        let query = sqlx::query_as!(
            DatabaseInteraction,
            "SELECT 
                app_permissions,
                application_id,
                channel_id,
                data as \"data: Json<InteractionData>\",
                guild_id,
                guild_locale,
                interaction_id,
                kind as \"kind: DatabaseInteractionKind\",
                locale,
                member_id,
                message_id,
                token,
                user_id
            FROM interactions WHERE interaction_id = $1",
            id
        );

        query.fetch_optional(&self.0).await
    }
}
