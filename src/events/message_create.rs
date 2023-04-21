use tracing::info;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::State;

pub async fn message_create_handler(_state: State, _msg: Box<MessageCreate>) -> anyhow::Result<()> {
    info!("Message created!");

    Ok(())
}
