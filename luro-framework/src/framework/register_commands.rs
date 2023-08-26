use std::collections::HashMap;

use anyhow::anyhow;
use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::info;
use twilight_interactions::command::ApplicationCommandData;
use twilight_model::{
    application::command::Command,
    id::{marker::GuildMarker, Id}
};

use crate::Framework;

impl<D: LuroDatabaseDriver> Framework<D> {
    /// Register the commands present in the framework
    pub async fn register_commands(&self, guild_id: Option<Id<GuildMarker>>) -> anyhow::Result<Vec<Command>> {
        Ok(match guild_id {
            Some(guild_id) => {
                let commands: Vec<Command> = match self.guild_commands.lock() {
                    Ok(guild_commands) => match guild_commands.get(&guild_id).cloned().map(|commands| commands.into_values()) {
                        Some(commands) => commands.map(|x|x.into()).collect(),
                        None => vec![]
                    },
                    Err(why) => return Err(anyhow!("Guild Commands mutex is poistioned: {why}"))
                };
                info!("Registering {} commands in guild {guild_id}!", commands.len());
                self.new_interaction_client()
                    .await?
                    .set_guild_commands(guild_id, &commands)
                    .await?
                    .model()
                    .await?
            }
            None => {
                let commands: Vec<Command> = match self.global_commands.lock() {
                    Ok(global_commands) => global_commands.values().cloned().map(|x|x.into()).collect(),
                    Err(why) => return Err(anyhow!("Guild Commands mutex is poistioned: {why}"))
                };
                info!("Registering {} global commands!", commands.len());

                self.new_interaction_client()
                    .await?
                    .set_global_commands(&commands)
                    .await?
                    .model()
                    .await?
                
            }
        })
    }

    /// Register new commands in the framework, does NOT register them discord's side. Call `register_commands` for that!
    pub async fn register_new_commands(
        &self,
        guild_id: Option<Id<GuildMarker>>,
        new_commands: Vec<ApplicationCommandData>
    ) -> anyhow::Result<&Self> {
        match guild_id {
            Some(guild_id) => match self.guild_commands.lock() {
                Ok(mut guild_commands) => {
                    let mut commands = HashMap::new();
                    for command in new_commands {
                        commands.insert(command.name.clone(), command);
                    }
                    guild_commands.insert(guild_id, commands);
                }
                Err(why) => return Err(anyhow!("Guild Commands mutex is poistioned: {why}"))
            },
            None => match self.global_commands.lock() {
                Ok(mut guild_commands) => {
                    for command in new_commands {
                        guild_commands.insert(command.name.clone(), command);
                    }
                }
                Err(why) => return Err(anyhow!("Guild Commands mutex is poistioned: {why}"))
            }
        };
        Ok(self)
    }
}
