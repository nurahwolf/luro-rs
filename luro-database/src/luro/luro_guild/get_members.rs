use std::sync::Arc;

use crate::{LuroDatabase, LuroGuild, LuroUser};

impl LuroGuild {
    pub async fn get_members(&self, db: Arc<LuroDatabase>) -> Result<Vec<LuroUser>, sqlx::Error> {
        db.get_members_of_guild(self.guild_id).await
    }
}
