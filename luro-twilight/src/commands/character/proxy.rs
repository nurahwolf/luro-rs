use luro_framework::{command::LuroCommandTrait, Framework, InteractionCommand, LuroInteraction};
use luro_model::database_driver::LuroDatabaseDriver;
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "proxy", desc = "Configure a prefix for proxying messages")]
pub struct Proxy {
    #[command(desc = "The character that should be modified", autocomplete = true)]
    name: String,
    /// The prefix to cause the proxy. e.g. "+n" so that "+n hi!" appears as the character
    prefix: String,
    /// Set to true to remove the prefix
    remove: Option<bool>,
}
#[async_trait::async_trait]

impl LuroCommandTrait for Proxy {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let user_id = interaction.author_id();
        let mut user_data = ctx.database.get_user(&user_id).await?;
        if user_data.characters.is_empty() {
            return interaction
                .respond(&ctx, |r| {
                    r.content(format!("Sorry, <@{user_id}> has no character profiles configured!"))
                        .ephemeral()
                })
                .await;
        }

        let character = match user_data.characters.get(&data.name) {
            Some(character) => character,
            None => {
                let mut characters = String::new();

                for (character_name, character) in user_data.characters {
                    writeln!(characters, "- {character_name}: {}", character.short_description)?
                }

                let response = format!("I'm afraid that user <@{user_id}> has no characters with the name `{}`! They do however, have the following profiles configured...\n{}", data.name, characters);
                return interaction.respond(&ctx, |r| r.content(response).ephemeral()).await;
            }
        };

        if data.remove.unwrap_or_default() {
            let content = match user_data.character_prefix.remove(&data.prefix) {
                Some(prefix) => {
                    ctx.database.modify_user(&user_id, &user_data).await?;
                    format!("Prefix {prefix} removed from character {}!", data.name)
                }
                None => {
                    let mut prefix_string = String::new();
                    for (prefix, character) in user_data.character_prefix {
                        writeln!(prefix_string, "- {prefix} - {character}")?
                    }
                    prefix_string
                }
            };
            return interaction.respond(&ctx, |r| r.content(content).ephemeral()).await;
        }

        user_data.character_prefix.insert(data.prefix, data.name.clone());
        ctx.database.modify_user(&user_id, &user_data).await?;

        let character_icon = match !character.icon.is_empty() {
            true => character.icon.clone(),
            false => user_data.avatar(),
        };

        interaction.respond(&ctx, |response|response.embed(|embed|embed.author(|author|author.icon_url(character_icon).name(&data.name)).description("Your proxied messages will look like this now!\n\n*Note:* If I am using your avatar, make sure that I have been set with an icon! `/character icon`")).ephemeral()).await
    }
}
