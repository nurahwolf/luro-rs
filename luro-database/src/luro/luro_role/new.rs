use std::sync::Arc;

use anyhow::Context;
use tracing::{error, warn};
use twilight_model::id::{marker::RoleMarker, Id};

use crate::{LuroDatabase, LuroRole};

impl LuroRole {
    pub async fn new(db: Arc<LuroDatabase>, role_id: Id<RoleMarker>) -> anyhow::Result<Self> {
        // TODO: Redo this
        // if let Ok(Some(role)) = self.database().get_role(role_id.get() as i64).await {
        //     return Ok(role.into());
        // }

        // let cache = self.cache();
        // let cached_role = match cache.role(role_id) {
        //     Some(role) => role,
        //     None => return Err(anyhow!("No role referance in cache or database: {role_id}")),
        // };

        // Ok(self
        //     .database()
        //     .update_role((cached_role.guild_id(), cached_role.resource().clone()))
        //     .await?
        //     .into())

        let role = db.get_role(&role_id).await?.unwrap().into();
        Ok(role)
    }
}
