use twilight_http::Response;
use twilight_model::channel::Message;

use crate::models::LuroResponse;

use super::LuroFramework;

impl LuroFramework {
    /// Send a message, useful if you do not want to consume the interaction.
    pub async fn send_message(&self, slash: &LuroResponse) -> anyhow::Result<Response<Message>> {
        let response = slash.clone();

        let mut message = self
            .twilight_client
            .create_message(slash.interaction.channel.as_ref().unwrap().id);

        if let Some(embeds) = &response.embeds {
            message = message.embeds(embeds)
        }

        if let Some(content) = &response.content {
            message = message.content(content)
        }

        if let Some(components) = &response.components {
            message = message.components(components)
        }

        if let Some(flags) = &response.flags {
            message = message.flags(*flags)
        }

        if let Some(interaction_message) = &slash.interaction.message {
            message = message.reply(interaction_message.id)
        }

        Ok(message.await?)
    }
}
