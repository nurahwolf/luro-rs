use luro_framework::{CommandInteraction, InteractionTrait, Luro, LuroCommand};
use rand::{seq::SliceRandom, thread_rng};
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "proxy", desc = "Configure a prefix for proxying messages")]
pub struct Proxy {
    #[command(desc = "The character that should be modified", autocomplete = true)]
    name: String,
    /// The prefix to cause the proxy. e.g. "+n" so that "+n hi!" appears as the character
    prefix: String,
    /// Set to true to remove the prefix
    remove: Option<bool>,
}

impl LuroCommand for Proxy {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let user_id = ctx.author.user_id();
        let user = ctx.fetch_user(&user_id).await?;
        let mut character = match user.fetch_character(&self.name).await? {
            Some(character) => character,
            None => {
                let mut characters = String::new();

                for (character_name, character) in user.fetch_characters().await? {
                    writeln!(characters, "- {character_name}: {}", character.sfw_summary)?
                }

                let response = format!("I'm afraid that user <@{}> has no characters with the name `{}`! They do however, have the following profiles configured...\n{}",user.user_id, self.name, characters);
                return ctx.respond(|r| r.content(response).ephemeral()).await;
            }
        };

        if self.remove.unwrap_or_default() {
            character.prefix = None;
            user.update_character(character).await?;
            return ctx
                .respond(|r| {
                    r.content(format!("Prefix `{}` removed from character {}!", self.prefix, self.name))
                        .ephemeral()
                })
                .await;
        }

        character.prefix = Some(self.prefix);
        user.update_character(character.clone()).await?;

        let accent_colour = ctx.accent_colour();
        let character_icon = character
            .sfw_icons
            .map(|x| x.choose(&mut thread_rng()).cloned())
            .map(|x| x.unwrap_or(user.avatar()))
            .unwrap_or(user.avatar());
        ctx.respond(|response|response.embed(|embed|embed.colour(accent_colour).author(|author|author.icon_url(character_icon).name(&self.name)).description("Your proxied messages will look like this now!\n\n*Note:* If I am using your avatar, make sure that I have been set with an icon! `/character icon`")).ephemeral()).await
    }
}
