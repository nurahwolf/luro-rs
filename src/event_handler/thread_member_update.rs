use tracing::info;
use twilight_model::gateway::payload::incoming::ThreadMemberUpdate;

use crate::framework::LuroFramework;

impl LuroFramework {
    pub async fn listener_thread_member_update(&self, event: Box<ThreadMemberUpdate>) -> anyhow::Result<()> {
        info!(thread = ?event, "Thread Member Update");

        Ok(())
    }
}
