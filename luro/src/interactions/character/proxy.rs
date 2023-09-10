use anyhow::Context;
use luro_model::database::drivers::LuroDatabaseDriver;
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::{interaction::LuroSlash, luro_command::LuroCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "proxy", desc = "Configure a prefix for proxying messages")]
pub struct Proxy {
    #[command(desc = "The character that should be modified", autocomplete = true)]
    name: String,
    /// The prefix to cause the proxy. e.g. "+n" so that "+n hi!" appears as the character
    prefix: String,
    /// Set to true to remove the prefix
    remove: Option<bool>
}

impl LuroCommand for Proxy {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let user_id = ctx
            .interaction
            .author_id()
            .context("Expected to find the user running this command")?;

        let mut user_data = ctx.framework.database.get_user(&user_id, false).await?;
        if user_data.characters.is_empty() {
            return ctx
                .respond(|r| {
                    r.content(format!("Sorry, <@{user_id}> has no character profiles configured!"))
                        .ephemeral()
                })
                .await;
        }

        let character = match user_data.characters.get(&self.name) {
            Some(character) => character,
            None => {
                let mut characters = String::new();

                for (character_name, character) in user_data.characters {
                    writeln!(characters, "- {character_name}: {}", character.short_description)?
                }

                let response = format!("I'm afraid that user <@{user_id}> has no characters with the name `{}`! They do however, have the following profiles configured...\n{}", self.name, characters);
                return ctx.respond(|r| r.content(response).ephemeral()).await;
            }
        };

        if self.remove.unwrap_or_default() {
            let content = match user_data.character_prefix.remove(&self.prefix) {
                Some(prefix) => {
                    ctx.framework.database.save_user(&user_id, &user_data).await?;
                    format!("Prefix {prefix} removed from character {}!", self.name)
                }
                None => {
                    let mut prefix_string = String::new();
                    for (prefix, character) in user_data.character_prefix {
                        writeln!(prefix_string, "- {prefix} - {character}")?
                    }
                    prefix_string
                }
            };
            return ctx.respond(|r| r.content(content).ephemeral()).await;
        }

        user_data.character_prefix.insert(self.prefix, self.name.clone());
        ctx.framework.database.save_user(&user_id, &user_data).await?;

        let character_icon = match !character.icon.is_empty() {
            true => character.icon.clone(),
            false => user_data.avatar()
        };

        ctx.respond(|response|response.embed(|embed|embed.author(|author|author.icon_url(character_icon).name(&self.name)).description("Your proxied messages will look like this now!\n\n*Note:* If I am using your avatar, make sure that I have been set with an icon! `/character icon`")).ephemeral()).await
    }
}
