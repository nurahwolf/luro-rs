use twilight_gateway::Intents;

pub struct Configuration {
    /// The token used for interacting with the Discord API
    pub token: String,
    /// The intents we want to listen for
    pub intents: Intents,
    /// The host for where Lavalink is running
    #[cfg(feature = "lavalink")]
    pub lavalink_host: String,
    /// The auth header for being able to interact with lavalink
    #[cfg(feature = "lavalink")]
    pub lavalink_auth: String,
}

impl Configuration {
    /// Regular configuration
    #[cfg(not(feature = "lavalink"))]
    pub fn new(intents: String, token: String,) -> Self {
        Self { intents, token, }
    }

    /// New configuration, with lavalink host details
    #[cfg(feature = "lavalink")]
    pub fn new(intents: Intents, token: String, lavalink_host: String, lavalink_auth: String,) -> Self {
        Self {
            token,
            intents,
            lavalink_host,
            lavalink_auth,
        }
    }
}
