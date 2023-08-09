use twilight_model::gateway::payload::incoming::ThreadUpdate;

use crate::framework::Framework;
use luro_model::luro_database_driver::LuroDatabaseDriver;

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn listener_thread_update(&self, event: Box<ThreadUpdate>) -> anyhow::Result<()> {
        self.response_thread_update(&event).await
    }
}
