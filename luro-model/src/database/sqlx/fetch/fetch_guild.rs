use twilight_model::id::{marker::GuildMarker, Id};

use crate::{
    database::sqlx::{Database, Error},
    guild::Guild,
};

impl Database {
    pub async fn fetch_guild(&self, _guild_id: Id<GuildMarker>) -> Result<Option<Guild>, Error> {
        todo!()
    }
}
