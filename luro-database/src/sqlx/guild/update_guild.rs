use crate::{sync::GuildSync, DatabaseGuild, LuroDatabase};

impl LuroDatabase {
    pub async fn update_guild(&self, guild: impl Into<GuildSync>) -> Result<DatabaseGuild, sqlx::Error> {
        let guild = match guild.into() {
            GuildSync::Guild(guild) => self.handle_guild(guild).await?,
            GuildSync::GuildUpdate(guild) => self.handle_guild_update(guild).await?,
            GuildSync::LuroGuild(guild) => self.handle_luro_guild(guild).await?,
        };

        Ok(guild)
    }
}
