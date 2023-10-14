use crate::{DatabaseGuild, DatabaseGuildType, LuroDatabase};

impl LuroDatabase {
    pub async fn update_guild(&self, guild: impl Into<DatabaseGuildType>) -> Result<DatabaseGuild, sqlx::Error> {
        let guild = guild.into();

        match guild {
            DatabaseGuildType::Guild(guild) => self.handle_guild(guild).await,
            DatabaseGuildType::GuildUpdate(guild) => self.handle_guild_update(guild).await,
            DatabaseGuildType::LuroGuild(guild) => self.handle_luro_guild(guild).await,
        }
    }
}
