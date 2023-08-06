use twilight_model::gateway::payload::incoming::ThreadUpdate;

use crate::framework::LuroFramework;

impl LuroFramework {
    pub async fn listener_thread_update(&self, event: Box<ThreadUpdate>) -> anyhow::Result<()> {
        self.response_thread_update(&event).await
    }
}
