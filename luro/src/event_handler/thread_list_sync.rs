use tracing::info;
use twilight_model::gateway::payload::incoming::ThreadListSync;

use crate::models::LuroFramework;

impl LuroFramework {
    pub async fn listener_thread_list_sync(&self, event: ThreadListSync) -> anyhow::Result<()> {
        info!(thread = ?event, "Thread created");

        Ok(())
    }
}
