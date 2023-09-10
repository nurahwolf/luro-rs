use tracing::info;
use twilight_model::gateway::payload::incoming::ThreadMembersUpdate;

use crate::framework::Framework;
use luro_model::database_driver::LuroDatabaseDriver;

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn listener_thread_members_update(&self, event: ThreadMembersUpdate) -> anyhow::Result<()> {
        info!(thread = ?event, "Thread created");

        Ok(())
    }
}
