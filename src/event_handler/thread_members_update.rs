use tracing::info;
use twilight_model::gateway::payload::incoming::ThreadMembersUpdate;

use crate::models::LuroFramework;

impl LuroFramework {
    pub async fn listener_thread_members_update(&self, event: ThreadMembersUpdate) -> anyhow::Result<()> {
        info!(thread = ?event, "Thread created");

        Ok(())
    }
}
