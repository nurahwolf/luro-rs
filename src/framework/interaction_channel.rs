use anyhow::anyhow;
use twilight_model::channel::Channel;

use crate::models::LuroResponse;

use super::LuroFramework;

impl LuroFramework {
    /// Get the interaction channel.
    pub fn channel(&self, slash: &LuroResponse) -> anyhow::Result<Channel> {
        slash
            .interaction
            .channel
            .clone()
            .ok_or_else(|| anyhow!("Unable to get the channel this interaction was ran in"))
    }
}
