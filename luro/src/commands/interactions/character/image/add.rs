use std::fmt::Write;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::models::interaction::{InteractionContext, InteractionResult};

#[derive(CommandModel, CreateCommand)]
#[command(name = "add", desc = "Set the primary image for this character")]
pub struct Command {
    #[command(desc = "The character to get", autocomplete = true)]
    pub character: String,
    /// The name of the image
    name: String,
    /// The URL the image should be set to
    url: String,
    /// Is this a NSFW image?
    nsfw: bool,
    /// Do you want this image to show up as one of the main images in your profile?
    fav: bool,
    /// Overwrite an image by ID
    overwrite: Option<i64>,
    /// The source URL for the image
    source: Option<String>,
}

impl crate::models::CreateCommand for Command {
    async fn handle_command(self, ctx: &mut InteractionContext) -> InteractionResult<()> {
        let character = match ctx.database.user_fetch_character(ctx.author.user_id, &self.name).await? {
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

        let img = CharacterImage {
            img_id: self.overwrite.unwrap_or_default(),
            name: self.name,
            nsfw: self.nsfw,
            owner_id: ctx.author.user_id.get() as i64,
            source: self.source,
            url: self.url,
            character_name: self.character,
            favourite: self.fav,
        };

        match self.overwrite {
            Some(_) => character.update_image(&img).await?,
            None => ctx.database.sqlx,
        };

        let mut embed = ctx.default_embed().await;
        embed
            .footer(|f| f.text(format!("Image ID: {}", img.img_id)))
            .image(|i| i.url(img.url))
            .author(|author| {
                author
                    .name(format!("Profile by {}", ctx.author.name()))
                    .icon_url(ctx.author.avatar_url())
            });

        if let Some(source) = &img.source {
            embed.url(source);
        }

        if !img.name.is_empty() {
            embed.title(img.name);
        }

        ctx.respond(|r| r.add_embed(embed).ephemeral()).await
    }
}
