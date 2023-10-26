use std::sync::Arc;

use twilight_model::id::{marker::RoleMarker, Id};

use crate::{LuroDatabase, LuroGuild, LuroRole};

impl LuroGuild {
    /// Fetch and return a [LuroGuild], updating the database if not present
    /// Luro Database -> Twilight Cache
    pub async fn fetch_role(&self, db: Arc<LuroDatabase>, role_id: Id<RoleMarker>) -> anyhow::Result<LuroRole> {
        LuroRole::new(db, self.guild_id, role_id).await
    }
}
