use twilight_model::{
    channel::Webhook,
    id::{
        marker::ChannelMarker,
        Id
    }
};

use crate::{LuroFramework, WEBHOOK_NAME};

/// Used for handling webhooks
pub struct LuroWebhook {
    framework: LuroFramework
}

impl LuroWebhook {
    pub async fn new(framework: &LuroFramework) -> anyhow::Result<Self> {
        Ok(Self { framework: framework.clone() })
    }

    // Get a webhook for a channel, or create it if it does not exist
    pub async fn get_webhook(self, channel_id: Id<ChannelMarker>) -> anyhow::Result<Webhook> {
        let webhooks = self
            .framework
            .twilight_client
            .channel_webhooks(channel_id)
            .await?
            .model()
            .await?;
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
            .framework
            .twilight_client
            .create_webhook(channel_id, WEBHOOK_NAME)
            .await?
            .model()
            .await?)
    }
}
