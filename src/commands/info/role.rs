use async_trait::async_trait;
use std::{fmt::Write, time::Duration};
use tracing::debug;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedMentionable, ResolvedUser};
use twilight_model::id::{
    marker::{GenericMarker, RoleMarker},
    Id
};
use twilight_util::{
    builder::embed::{EmbedFieldBuilder, ImageSource},
    snowflake::Snowflake
};

use crate::{models::LuroSlash, traits::luro_functions::LuroFunctions};

use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand)]
#[command(name = "role", desc = "Information about a role")]
pub struct InfoRole {
    /// The role to get
    role: ResolvedMentionable
}

#[async_trait]
impl LuroCommand for InfoRole {
    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let mut embed;
        {
            let role = match ctx.luro.twilight_cache.role(self.role.id().cast()) {
                Some(role) => role,
                None => return ctx.clone().content("Role not found!").ephemeral().respond().await
            };
            embed = ctx.default_embed().await?;
            let mut description: String = String::new();

            embed = embed.title(&role.name);

            for guild_role in ctx.luro.twilight_client.roles(role.guild_id()).await?.model().await?.iter() {
                if guild_role.id == role.id {
                    writeln!(description, "--> <@&{}> <--", guild_role.id)?;
                    continue;
                }
                writeln!(description, "<@&{}>", guild_role.id)?;
            }

            embed = embed.description(description)
        }

        ctx.embed(embed.build())?.respond().await
    }
}
