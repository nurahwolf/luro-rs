use std::sync::Arc;

use twilight_model::id::{marker::GuildMarker, Id};

use crate::{LuroDatabase, LuroGuild};

impl LuroGuild {
    pub async fn new(db: Arc<LuroDatabase>, guild_id: Id<GuildMarker>) -> anyhow::Result<Self> {
        db.get_guild(guild_id).await
    }
}
