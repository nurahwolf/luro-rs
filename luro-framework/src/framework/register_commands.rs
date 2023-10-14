use std::collections::HashMap;

use anyhow::anyhow;
use tracing::info;
use twilight_model::{
    application::command::Command,
    id::{marker::GuildMarker, Id},
};

use crate::{slash_command::LuroCommand, Framework, Luro};

impl Framework {
    /// Register the commands present in the framework
    pub async fn register_commands(&self, guild_id: Option<Id<GuildMarker>>) -> anyhow::Result<Vec<Command>> {
        Ok(match guild_id {
            Some(guild_id) => {
                let commands: Vec<Command> = match self.guild_commands.lock() {
                    Ok(guild_commands) => match guild_commands.get(&guild_id).map(|commands| commands.values()) {
                        Some(commands) => commands.map(|x| ((x.create)()).into()).collect(),
                        None => vec![],
                    },
                    Err(why) => return Err(anyhow!("Guild Commands mutex is poistioned: {why}")),
                };
                info!("Registering {} commands in guild {guild_id}!", commands.len());
                self.interaction_client()
                    .await?
                    .set_guild_commands(guild_id, &commands)
                    .await?
                    .model()
                    .await?
            }
            None => {
                let commands: Vec<Command> = match self.global_commands.lock() {
                    Ok(global_commands) => global_commands.values().map(|x| ((x.create)()).into()).collect(),
                    Err(why) => return Err(anyhow!("Guild Commands mutex is poistioned: {why}")),
                };
                info!("Registering {} global commands!", commands.len());

                self.interaction_client()
                    .await?
                    .set_global_commands(&commands)
                    .await?
                    .model()
                    .await?
            }
        })
    }

    /// Register a single command in the framework, does NOT register them discord's side. Call `register_commands` for that!
    pub async fn register_new_command(&self, guild_id: Option<Id<GuildMarker>>, new_commands: LuroCommand) -> anyhow::Result<&Framework> {
        match guild_id {
            Some(guild_id) => match self.guild_commands.lock() {
                Ok(mut guild_commands) => {
                    let mut commands = HashMap::new();
                    commands.insert(new_commands.name.to_string(), new_commands);
                    guild_commands.insert(guild_id, commands);
                }
                Err(why) => return Err(anyhow!("Guild Commands mutex is poistioned: {why}")),
            },
            None => match self.global_commands.lock() {
                Ok(mut guild_commands) => {
                    guild_commands.insert(new_commands.name.to_string(), new_commands);
                }
                Err(why) => return Err(anyhow!("Guild Commands mutex is poistioned: {why}")),
            },
        };
        Ok(self)
    }

    /// Register new commands in the framework, does NOT register them discord's side. Call `register_commands` for that!
    pub async fn register_new_commands(
        &self,
        guild_id: Option<Id<GuildMarker>>,
        new_commands: Vec<LuroCommand>,
    ) -> anyhow::Result<&Framework> {
        match guild_id {
            Some(guild_id) => match self.guild_commands.lock() {
                Ok(mut guild_commands) => {
                    let mut commands = HashMap::new();
                    for command in new_commands {
                        commands.insert(command.name.to_string(), command);
                    }
                    guild_commands.insert(guild_id, commands);
                }
                Err(why) => return Err(anyhow!("Guild Commands mutex is poistioned: {why}")),
            },
            None => match self.global_commands.lock() {
                Ok(mut guild_commands) => {
                    for command in new_commands {
                        guild_commands.insert(command.name.to_string(), command);
                    }
                }
                Err(why) => return Err(anyhow!("Guild Commands mutex is poistioned: {why}")),
            },
        };
        Ok(self)
    }
}
