use twilight_model::gateway::payload::incoming::ThreadCreate;

use crate::framework::Framework;
use luro_model::luro_database_driver::LuroDatabaseDriver;

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn listener_thread_create(&self, event: Box<ThreadCreate>) -> anyhow::Result<()> {
        self.response_thread_created(&event).await
    }
}
