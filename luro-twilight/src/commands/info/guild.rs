use std::fmt::Write;

use luro_framework::{CommandInteraction, Luro, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    http::attachment::Attachment,
    id::{marker::GenericMarker, Id},
};

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "guild", desc = "Information about a guild")]
pub struct Guild {
    /// Get the details of a different guild
    guild: Option<Id<GenericMarker>>,
    /// Set this if you want a copy of your data (GUILD OWNER ONLY).
    gdpr_export: Option<bool>,
}

impl LuroCommand for Guild {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let guild = match self.guild {
            Some(guild_requested) => ctx.get_guild(&guild_requested.cast()).await?,
            None => match &ctx.guild {
                Some(guild) => guild.clone(),
                None => return ctx.response_simple(luro_framework::Response::NotGuild).await,
            },
        };
        let mut luro_guild = String::new();
        let mut guild_description = String::new();

        let mut embed = ctx.default_embed().await;
        embed.title(&guild.name);

        writeln!(luro_guild, "- Guild Name: {}", &guild.name)?;
        // if !guild.commands.is_empty() {
        //     writeln!(luro_guild, "- Guild Commands: {:#?}", guild.commands)?;
        // }

        writeln!(guild_description, "- Owner: <@{}>", guild.owner_id)?;
        writeln!(guild_description, "- AFK Timeout: {} seconds", guild.afk_timeout.get())?;
        embed.description(guild_description);
        embed.create_field("Luro Settings", &luro_guild, false);

        ctx.respond(|response| {
            if self.gdpr_export.unwrap_or_default() {
                if guild.owner_id != ctx.author.user_id() {
                    response.content(format!(
                        "Hey <@{}>! <@{}> is being a cunt and trying to steal your guild data!",
                        guild.owner_id,
                        ctx.author.user_id()
                    ));
                } else if let Ok(guild_settings) = toml::to_string_pretty(&guild) {
                    response.attachments(
                        vec![Attachment::from_bytes(
                            format!("gdpr-export-{}.txt", ctx.author.user_id()),
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
