use tokio::{fs::File, io::AsyncReadExt};

const ACCENT_COLOUR: u32 = 0xDABEEF;

#[derive(thiserror::Error, Debug)]
pub enum GatewayConfigError {
    #[error("Tokio had a problem modifying the configuration")]
    FileIOError(#[from] std::io::Error),
    #[error("There was an error with the TOML content of the configuration")]
    TomlError(#[from] toml::de::Error),
}

#[derive(Debug, Default, ::serde::Deserialize, ::serde::Serialize, Clone)]
pub struct Config {
    pub accent_colour: Option<u32>,
    pub bot_name: Option<String>,
    pub connection_string: Option<String>,
    pub description: Option<String>,
    pub e621_blacklist: Option<String>,
    pub e621_useragent: Option<String>,
    pub webhook_name: Option<String>,
    pub git_url: Option<String>,
    pub commands_enabled: Option<Vec<String>>,
    pub commands_disabled: Option<Vec<String>>,
    pub prefix: Option<String>,
}

impl Config {
    /// Returns true if a command is allowed to run.
    ///
    /// commands_enabled = None && commands_disabled = None                     | true
    /// commands_enabled = None && commands_disabled = Included                 | false
    /// commands_enabled = None && commands_disabled = Not Included             | true
    /// commands_enabled = Included && commands_disabled = None                 | true
    /// commands_enabled = Included && commands_disabled = Included             | false
    /// commands_enabled = Included && commands_disabled = Not Included         | true
    /// commands_enabled = Not Included  && commands_disabled = None            | false
    /// commands_enabled = Not Included  && commands_disabled = Included        | false
    /// commands_enabled = Not Included  && commands_disabled = Not Included    | false
    pub fn command_allowed(&self, command: &str) -> bool {
        match &self.commands_enabled {
            // If commands_enabled is present (Included or Not Included)
            Some(commands) => {
                match commands.contains(&command.to_string()) {
                    // commands_enabled = Included, check to see if blacklisted
                    true => !self.command_disabled(command), // This returns 'true' if the command is disabled. Inverse it to say that its allowed to run.
                    // commands_enabled = Not Included, always return false
                    false => false,
                }
            }
            // If commands_enabled is NOT present (None)
            None => self.command_disabled(command),
        }
    }

    /// Returns true if a command is enabled.
    pub fn command_enabled(&self, command: &str) -> bool {
        match &self.commands_enabled {
            Some(commands) => commands.contains(&command.to_string()),
            None => true,
        }
    }

    /// Returns true if a command is disabled.
    pub fn command_disabled(&self, command: &str) -> bool {
        match &self.commands_disabled {
            Some(commands) => commands.contains(&command.to_string()),
            None => false,
        }
    }

    /// Fetch the configuration from disk
    pub async fn fetch(file_name: &str) -> Result<Self, GatewayConfigError> {
        let mut file_contents = String::new();

        // Attempts to open the file, if the file is not found, create it
        let mut file = match File::open(file_name).await {
            Ok(file) => file,
            Err(why) => match why.kind() {
                std::io::ErrorKind::NotFound => File::create(file_name).await?,
                _ => return Err(why.into()),
            },
        };

        // Attempt to read the file to a string
        if let Ok(size) = file.read_to_string(&mut file_contents).await {
            tracing::debug!("Read file {file_name} of length {size}")
        };

        Ok(toml::from_str::<Self>(&file_contents)?)
    }

    /// Sync the configuration to disk.
    /// If no new config is passed, then the current config will be updated from whatever is on disk.
    /// If a new config is passed, it is written to disk and the in-memory config updated to whatever was passed.
    pub async fn snyc_config(
        &mut self,
        new_config: Option<Self>,
        file_name: &str,
    ) -> anyhow::Result<&mut Self> {
        // Handle new config
        if let Some(new_config) = new_config {
            let toml_string = match toml::to_string(&new_config) {
                Ok(string) => string,
                Err(why) => {
                    return Err(anyhow::anyhow!(
                        "Error serialising struct to toml string: {why}"
                    ))
                }
            };

            if let Err(why) = tokio::fs::write(file_name, toml_string).await {
                tracing::error!(?why, "Failed to write toml_string to file. Changes will be lost as they have only been changed in memory!");
            }

            *self = new_config;
            return Ok(self);
        }

        // Else fetch from disk
        *self = Self::fetch(file_name).await?;
        Ok(self)
    }

    pub fn accent_colour(&self) -> u32 {
        self.accent_colour.unwrap_or(ACCENT_COLOUR)
    }
}
