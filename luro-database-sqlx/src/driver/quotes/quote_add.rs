use twilight_model::id::{
    marker::{MessageMarker, UserMarker},
    Id,
};

impl crate::SQLxDriver {
    /// Add a quote to the database, returning the added quote ID
    pub async fn quote_add(
        &self,
        added_by: Id<UserMarker>,
        message: &luro_model::types::Message,
        nsfw: bool,
    ) -> anyhow::Result<Id<MessageMarker>> {
        Ok(Id::new(
            sqlx::query!(
                "
                INSERT INTO quotes (added_by, channel_id, message_id, nsfw)
                VALUES ($1, $2, $3, $4)
                RETURNING message_id
            ",
                added_by.get() as i64,
                message.id.get() as i64,
                message.channel_id.get() as i64,
                nsfw
            )
            .fetch_one(&self.pool)
            .await?
            .message_id as u64,
        ))
    }
}
