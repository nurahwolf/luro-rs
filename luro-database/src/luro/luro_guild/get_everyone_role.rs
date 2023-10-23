use std::sync::Arc;

use crate::{LuroDatabase, LuroGuild, LuroRole};

impl LuroGuild {
    pub async fn get_everyone_role(&self, db: Arc<LuroDatabase>) -> anyhow::Result<LuroRole> {
        LuroRole::new(db, self.guild_id(), self.guild_id().cast()).await
    }
}
