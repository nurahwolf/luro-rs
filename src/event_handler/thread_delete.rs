use twilight_model::gateway::payload::incoming::ThreadDelete;

use crate::framework::LuroFramework;

impl LuroFramework {
    pub async fn listener_thread_delete(&self, event: ThreadDelete) -> anyhow::Result<()> {
        self.response_thread_deleted(&event).await
    }
}
