use twilight_model::gateway::payload::incoming::ThreadDelete;

use crate::framework::Framework;
use luro_model::database_driver::LuroDatabaseDriver;

impl<D: LuroDatabaseDriver> Framework {
    pub async fn listener_thread_delete(&self, event: ThreadDelete) -> anyhow::Result<()> {
        self.response_thread_deleted(&event).await
    }
}
