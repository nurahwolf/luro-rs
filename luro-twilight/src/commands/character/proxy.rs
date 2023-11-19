use luro_framework::{CommandInteraction, Luro, LuroCommand};
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "proxy", desc = "Configure a prefix for proxying messages")]
pub struct Proxy {
    #[command(desc = "The character that should be modified", autocomplete = true)]
    pub name: String,
    /// The prefix to cause the proxy. e.g. "+n" so that "+n hi!" appears as the character
    prefix: String,
    /// Set to true to remove the prefix
    remove: Option<bool>,
}

impl LuroCommand for Proxy {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let mut character = match ctx.database.user_fetch_character(ctx.author.user_id, &self.name).await? {
            Some(character) => character,
            None => {
                let mut characters = String::new();

                for character in ctx.database.user_fetch_characters(ctx.author.user_id).await? {
                    writeln!(characters, "- {}: {}", character.name, character.sfw_summary)?
                }

                let response = format!("I'm afraid that user <@{}> has no characters with the name `{}`! They do however, have the following profiles configured...\n{}",ctx.author.user_id, self.name, characters);
                return ctx.respond(|r| r.content(response).ephemeral()).await;
            }
        };

        if self.remove.unwrap_or_default() {
            character.prefix = None;
            ctx.database.driver.character_update(&character, ctx.author.user_id).await?;
            return ctx
                .respond(|r| {
                    r.content(format!("Prefix `{}` removed from character {}!", self.prefix, self.name))
                        .ephemeral()
                })
                .await;
        }

        character.prefix = Some(self.prefix);
        ctx.database.driver.character_update(&character, ctx.author.user_id).await?;

        let accent_colour = ctx.accent_colour();
        let character_icon = match ctx.channel.nsfw.unwrap_or_default() {
            true => character.nsfw_icon.unwrap_or(character.sfw_icon),
            false => character.sfw_icon,
        };

        ctx.respond(|response|response.embed(|embed|embed.colour(accent_colour).author(|author|author.icon_url(character_icon).name(&self.name)).description("Your proxied messages will look like this now!\n\n*Note:* If I am using your avatar, make sure that I have been set with an icon! `/character icon`")).ephemeral()).await
    }
}
