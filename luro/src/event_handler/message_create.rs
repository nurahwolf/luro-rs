use std::sync::Arc;

use anyhow::Error;
use luro_model::luro_database_driver::LuroDatabaseDriver;
use tracing::error;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{framework::Framework, models::LuroWebhook};

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn message_create_listener(self: &Arc<Self>, message: MessageCreate) -> Result<(), Error> {
        let user_data = self.database.get_user(&message.author.id).await?;
        let first_word = message.content.split(' ').next().unwrap_or("");
        if let Some(character_name) = user_data.character_prefix.get(first_word) {
            let character = match user_data.characters.get(character_name) {
                Some(character) => character,
                None => return Ok(())
            };
            let character_icon = match !character.icon.is_empty() {
                true => character.icon.clone(),
                false => user_data.avatar()
            };

            let luro_webhook = LuroWebhook::new(self.clone());
            let webhook = luro_webhook.get_webhook(message.channel_id).await?;
            let webhook_token = match webhook.token {
                Some(token) => token,
                None => match self.twilight_client.webhook(webhook.id).await?.model().await?.token {
                    Some(token) => token,
                    None => {
                        error!(
                            "I cannot setup a webhook in channel {} in response to message {}",
                            message.channel_id, message.id
                        );
                        return Ok(());
                    }
                }
            };

            let proxied_message = message.content.replace(first_word, "");

            self.twilight_client.delete_message(message.channel_id, message.id).await?;
            self.twilight_client
                .execute_webhook(webhook.id, &webhook_token)
                .username(&format!(
                    "{character_name} [{}]",
                    user_data.member_name(&message.guild_id)
                ))
                .content(&proxied_message)
                .avatar_url(&character_icon)
                .await?;

        }

        self.response_message_modified(&message.clone().into()).await?;

        Ok(())
    }
}
