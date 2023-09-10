use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::info;
use twilight_model::gateway::payload::incoming::ThreadListSync;

use crate::framework::Framework;

impl<D: LuroDatabaseDriver,> Framework<D,> {
    pub async fn listener_thread_list_sync(&self, event: ThreadListSync,) -> anyhow::Result<(),> {
        info!(thread = ?event, "Thread created");

        Ok((),)
    }
}
