use async_trait::async_trait;
use luro_framework::{command::ExecuteLuroCommand, interactions::InteractionTrait, CommandInteraction};
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "icon", desc = "Set the primary icon for this character")]
pub struct Icon {
    #[command(desc = "The character that should be modified", autocomplete = true)]
    name: String,
    /// The URL the icon should be set to
    icon: String,
    /// The URL a NSFW icon
    nsfw_icon: Option<String>,
}
#[async_trait::async_trait]
#[async_trait]
impl ExecuteLuroCommand for Icon {
    async fn interaction_command(&self, ctx: CommandInteraction<()>) -> anyhow::Result<()> {
        let user_id = ctx.author_id();

        let mut user_data = ctx.database.get_user(&user_id).await?;
        if user_data.characters.is_empty() {
            return ctx
                .respond(|r| {
                    r.content(format!("Sorry, <@{user_id}> has no character profiles configured!"))
                        .ephemeral()
                })
                .await;
        }

        match user_data.characters.get_mut(&self.name) {
            Some(character) => {
                character.nsfw_icon = self.nsfw_icon.clone();
                character.icon = self.icon.clone()
            }
            None => {
                let mut characters = String::new();

                for (character_name, character) in user_data.characters {
                    writeln!(characters, "- {character_name}: {}", character.short_description)?
                }

                let response = format!("I'm afraid that user <@{user_id}> has no characters with the name `{}`! They do however, have the following profiles configured...\n{}", self.name, characters);
                return ctx.respond(|r| r.content(response).ephemeral()).await;
            }
        };

        ctx.database.modify_user(&user_id, &user_data).await?;

        ctx.respond(|r| r.content("Done!").ephemeral()).await
    }
}
