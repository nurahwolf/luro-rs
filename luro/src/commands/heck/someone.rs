use std::convert::TryInto;

use async_trait::async_trait;
use tracing::{debug, trace};

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::id::Id;
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedFooterBuilder};

use crate::{
    commands::heck::{format_heck, get_heck},
    models::SlashUser,
    slash::Slash,
    traits::luro_command::LuroCommand
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
    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
        // Is the channel the interaction called in NSFW?
        let nsfw = ctx.channel()?.nsfw.unwrap_or(false);

        debug!("attempting to get a heck");
        let (heck, heck_id) = get_heck(&ctx.framework, self.id, ctx.interaction.guild_id, self.global, nsfw).await?;

        debug!("attempting to format the returned heck");
        let formatted_heck = format_heck(&heck, &ctx.author()?, &self.user.resolved).await;

        // This first attempts to get them from the guild if they are a member, otherwise resorts to fetching their user.
        let slash_author = match ctx.interaction.guild_id {
            Some(guild_id) => {
                match SlashUser::client_fetch_member(&ctx.framework, guild_id, Id::new(heck.author_id.get())).await {
                    Ok(slash_author) => slash_author.1,
                    Err(_) => {
                        SlashUser::client_fetch_user(&ctx.framework, Id::new(heck.author_id.get()))
                            .await?
                            .1
                    }
                }
            }
            None => {
                SlashUser::client_fetch_user(&ctx.framework, Id::new(heck.author_id.get()))
                    .await?
                    .1
            }
        };

        let embed_author = EmbedAuthorBuilder::new(format!("Heck created by {}", slash_author.name))
            .icon_url(slash_author.try_into()?)
            .build();

        // Create our response, depending on if the user wants a plaintext heck or not
        debug!("creating our response");
        if let Some(plaintext) = self.plaintext && plaintext {
            trace!("user wanted plaintext");
            ctx.content(formatted_heck.heck_message).respond().await
        } else {
            trace!("user wanted embed");
            let mut embed = ctx.default_embed().await?
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
            ctx.content(format!("<@{}>", self.user.resolved.id)).embed(embed.build())?.respond().await
        }
    }
}
