use luro_model::database_driver::LuroDatabaseDriver;
use twilight_model::gateway::payload::incoming::ThreadCreate;

use crate::framework::Framework;

impl<D: LuroDatabaseDriver> Framework {
    pub async fn listener_thread_create(&self, event: Box<ThreadCreate>) -> anyhow::Result<()> {
        self.response_thread_created(&event).await
    }
}
