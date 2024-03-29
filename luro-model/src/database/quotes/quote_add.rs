use twilight_model::id::{
    marker::UserMarker,
    Id,
};

impl crate::SQLxDriver {
    /// Add a quote to the database, returning the added quote ID
    pub async fn quote_add(&self, added_by: Id<UserMarker>, message: &luro_model::types::Message, nsfw: bool) -> anyhow::Result<i64> {
        tracing::info!(
            "Attempting to add message_id: {} - channel_id: {} to quotes",
            message.id,
            message.channel_id
        );

        Ok(sqlx::query!(
            "
                    INSERT INTO quotes (added_by, channel_id, message_id, nsfw)
                    VALUES ($1, $2, $3, $4)
                    RETURNING id
                ",
            added_by.get() as i64,
            message.channel_id.get() as i64,
            message.id.get() as i64,
            nsfw
        )
        .fetch_one(&self.pool)
        .await?
        .id)
    }
}
