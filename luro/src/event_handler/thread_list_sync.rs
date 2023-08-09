use luro_model::luro_database_driver::LuroDatabaseDriver;
use tracing::info;
use twilight_model::gateway::payload::incoming::ThreadListSync;

use crate::framework::Framework;

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn listener_thread_list_sync(&self, event: ThreadListSync) -> anyhow::Result<()> {
        info!(thread = ?event, "Thread created");

        Ok(())
    }
}
