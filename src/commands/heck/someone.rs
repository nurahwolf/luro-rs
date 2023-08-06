use std::convert::TryInto;

use async_trait::async_trait;
use tracing::{debug, trace};

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::id::Id;
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedFooterBuilder};

use crate::{
    commands::heck::{format_heck, get_heck},
    models::{LuroResponse, SlashUser},
    traits::luro_command::LuroCommand,
    LuroContext
};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "someone", desc = "Heck a user", dm_permission = true)]
pub struct HeckSomeoneCommand {
    /// The user to heck
    pub user: ResolvedUser,
    /// Get a global heck, or a heck that is specific to this server
    pub global: bool,
    /// Get a specific heck
    pub id: Option<i64>,
    /// Should the heck be sent as plaintext? (Without an embed)
    pub plaintext: Option<bool>
}

#[async_trait]
impl LuroCommand for HeckSomeoneCommand {
    async fn run_command(self, ctx: &LuroContext, mut slash: LuroResponse) -> anyhow::Result<()> {
        let (author, _) = ctx.get_interaction_author(&slash)?;

        // Is the channel the interaction called in NSFW?
        let nsfw = ctx.channel(&slash)?.nsfw.unwrap_or(false);

        debug!("attempting to get a heck");
        let (heck, heck_id) = get_heck(ctx, self.id, slash.interaction.guild_id, self.global, nsfw).await?;

        debug!("attempting to format the returned heck");
        let formatted_heck = format_heck(&heck, author, &self.user.resolved).await;

        // This first attempts to get them from the guild if they are a member, otherwise resorts to fetching their user.
        let slash_author = match slash.interaction.guild_id {
            Some(guild_id) => match SlashUser::client_fetch_member(ctx, guild_id, Id::new(heck.author_id)).await {
                Ok(slash_author) => slash_author.1,
                Err(_) => SlashUser::client_fetch_user(ctx, Id::new(heck.author_id)).await?.1
            },
            None => SlashUser::client_fetch_user(ctx, Id::new(heck.author_id)).await?.1
        };

        let embed_author = EmbedAuthorBuilder::new(format!("Heck created by {}", slash_author.name))
            .icon_url(slash_author.try_into()?)
            .build();

        // Create our response, depending on if the user wants a plaintext heck or not
        debug!("creating our response");
        if let Some(plaintext) = self.plaintext && plaintext {
            trace!("user wanted plaintext");
            slash.content(formatted_heck.heck_message);ctx.respond(&mut slash).await
        } else {
            trace!("user wanted embed");
            let mut embed = ctx.default_embed(&slash.interaction.guild_id)
            .description(formatted_heck.heck_message)
            .author(embed_author);
        if nsfw {
            embed = embed.footer(EmbedFooterBuilder::new(format!(
                "Heck ID {heck_id} - NSFW Heck"
            )))
        } else {
            embed = embed.footer(EmbedFooterBuilder::new(format!(
                "Heck ID {heck_id} - SFW Heck"
            )))
        }
            slash.content(format!("<@{}>", self.user.resolved.id)).embed(embed.build())?;
ctx.respond(&mut slash).await
        }
    }
}
