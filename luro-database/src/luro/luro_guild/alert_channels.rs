use crate::{GuildAlertChannels, LuroGuild};

impl LuroGuild {
    pub async fn alert_channels(&self) -> anyhow::Result<GuildAlertChannels> {
        Ok(GuildAlertChannels::default())
    }
}
