use crate::interactions::LuroDatabaseDriver;
use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    http::attachment::Attachment,
    id::{marker::GenericMarker, Id},
};

use crate::interaction::LuroSlash;

use crate::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand)]
#[command(name = "guild", desc = "Information about a guild")]
pub struct Guild {
    /// Get the details of a different guild
    guild: Option<Id<GenericMarker>>,
    /// Set this if you want a copy of your data (GUILD OWNER ONLY).
    gdpr_export: Option<bool>,
}

impl LuroCommand for Guild {
    async fn run_command(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let mut luro_guild = String::new();
        let mut guild_description = String::new();
        let guild_id = match self.guild {
            Some(guild) => guild.cast(),
            None => ctx.interaction.guild_id.unwrap(),
        };
        let guild = ctx.framework.twilight_client.guild(guild_id).await?.model().await?;
        let guild_settings = ctx.framework.database.get_guild(&guild_id).await?;
        let mut embed = ctx.default_embed().await;
        embed.title(&guild_settings.name);

        writeln!(luro_guild, "- Guild Name: {}", &guild_settings.name)?;
        if !guild_settings.commands.is_empty() {
            writeln!(luro_guild, "- Guild Commands: {:#?}", guild_settings.commands)?;
        }

        writeln!(guild_description, "- Owner: <@{}>", guild.owner_id)?;
        writeln!(guild_description, "- AFK Timeout: {} seconds", guild.afk_timeout.get())?;
        embed.description(guild_description);
        embed.create_field("Luro Settings", &luro_guild, false);

        ctx.respond(|response| {
            if self.gdpr_export.unwrap_or_default() {
                if guild.owner_id != ctx.interaction.author_id().unwrap() {
                    response.content(format!(
                        "Hey <@{}>! <@{}> is being a cunt and trying to steal your guild data!",
                        guild.owner_id,
                        ctx.interaction.author_id().unwrap()
                    ));
                } else if let Ok(guild_settings) = toml::to_string_pretty(&guild_settings) {
                    response.attachments(
                        vec![Attachment::from_bytes(
                            format!("gdpr-export-{}.txt", ctx.interaction.author_id().unwrap()),
                            guild_settings.as_bytes().to_vec(),
                            1,
                        )]
                        .into_iter(),
                    );
                }
            }
            response.add_embed(embed)
        })
        .await
    }
}
