use std::sync::Arc;

use crate::{DbMember, LuroDatabase, LuroGuild};

impl LuroGuild {
    pub async fn get_members(&self, db: Arc<LuroDatabase>) -> Result<Vec<DbMember>, sqlx::Error> {
        db.get_members_of_guild(self.guild_id).await
    }
}
