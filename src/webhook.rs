use twilight_model::{
    channel::Webhook,
    guild::Guild,
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id
    }
};

use crate::{LuroContext, WEBHOOK_NAME};

/// Used for handling webhooks
pub struct LuroWebhook {
    luro: LuroContext,
    guild: Guild
}

impl LuroWebhook {
    pub async fn new(luro: LuroContext, guild_id: Id<GuildMarker>) -> anyhow::Result<Self> {
        let guild = luro.twilight_client.guild(guild_id).await?.model().await?;

        Ok(Self { luro, guild })
    }

    // Get a webhook for a channel, or create it if it does not exist
    pub async fn get_webhook(self, channel_id: Id<ChannelMarker>) -> anyhow::Result<Webhook> {
        let webhooks = self.luro.twilight_client.channel_webhooks(channel_id).await?.model().await?;
        let mut webhook = None;

        for wh in webhooks {
            if let Some(ref webhook_name) = wh.name {
                if webhook_name == WEBHOOK_NAME {
                    webhook = Some(wh);
                    break;
                }
            }
        }

        match webhook {
            Some(webhook) => Ok(webhook),
            None => self.create_webhook(channel_id).await
        }
    }

    pub async fn create_webhook(self, channel_id: Id<ChannelMarker>) -> anyhow::Result<Webhook> {
        Ok(self
            .luro
            .twilight_client
            .create_webhook(channel_id, WEBHOOK_NAME)?
            .await?
            .model()
            .await?)
    }
}
