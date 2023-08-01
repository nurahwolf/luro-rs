use twilight_model::gateway::payload::incoming::ThreadCreate;

use crate::models::LuroFramework;

impl LuroFramework {
    pub async fn listener_thread_create(&self, event: Box<ThreadCreate>) -> anyhow::Result<()> {
        self.response_thread_created(&event).await
    }
}
