use twilight_model::gateway::payload::incoming::ThreadDelete;

use crate::framework::Framework;
use luro_model::database::drivers::LuroDatabaseDriver;

impl<D: LuroDatabaseDriver,> Framework<D,> {
    pub async fn listener_thread_delete(&self, event: ThreadDelete,) -> anyhow::Result<(),> {
        self.response_thread_deleted(&event,).await
    }
}
