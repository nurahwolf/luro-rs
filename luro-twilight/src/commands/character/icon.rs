use anyhow::Context;
use luro_framework::{CommandInteraction, InteractionTrait, Luro, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};

use super::character_response;

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

impl LuroCommand for Icon {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let user_id = ctx.author_id();
        let user = ctx.fetch_user(&user_id).await?;
        let character = user.fetch_character(&self.name).await?.context("No character with that name!")?;
        let nsfw = ctx.channel.nsfw.unwrap_or_default();

        character_response(ctx, &character, &user, nsfw).await
    }
}
