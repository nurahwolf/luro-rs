impl super::InteractionContext {
    pub async fn accent_colour(&self) -> u32 {
        tracing::warn!("Incomplete function called, please complete me!");
        if let Some(guild_id) = self.interaction.guild_id {
            if let Ok(guild) = self.gateway.twilight_client.guild(guild_id).await {
                if let Ok(_guild) = guild.model().await {
                    return crate::ACCENT_COLOUR;
                }
            }
        }

        crate::ACCENT_COLOUR
    }
}
