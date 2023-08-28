use luro_framework::{command::LuroCommand, Framework, InteractionCommand, LuroInteraction};
use luro_model::database::drivers::LuroDatabaseDriver;
use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "icon", desc = "Set the primary icon for this character")]
pub struct Icon {
    #[command(desc = "The character that should be modified", autocomplete = true)]
    name: String,
    /// The URL the icon should be set to
    icon: String,
    /// The URL a NSFW icon
    nsfw_icon: Option<String>
}

impl LuroCommand for Icon {
    async fn interaction_command<D: LuroDatabaseDriver>(
        self,
        ctx: Framework<D>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
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

        match user_data.characters.get_mut(&self.name) {
            Some(character) => {
                character.nsfw_icon = self.nsfw_icon;
                character.icon = self.icon
            }
            None => {
                let mut characters = String::new();

                for (character_name, character) in user_data.characters {
                    writeln!(characters, "- {character_name}: {}", character.short_description)?
                }

                let response = format!("I'm afraid that user <@{user_id}> has no characters with the name `{}`! They do however, have the following profiles configured...\n{}", self.name, characters);
                return interaction.respond(&ctx, |r| r.content(response).ephemeral()).await;
            }
        };

        ctx.database.save_user(&user_id, &user_data).await?;

        interaction.respond(&ctx, |r| r.content("Done!").ephemeral()).await
    }
}
