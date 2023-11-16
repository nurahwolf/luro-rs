use std::fmt::Write;

use luro_framework::{CommandInteraction, Luro, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{http::attachment::Attachment, id::Id};

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "guild", desc = "Information about a guild")]
pub struct Guild {
    /// Get the details of a different guild
    guild: Option<String>,
    /// Set this if you want a copy of your data (GUILD OWNER ONLY).
    gdpr_export: Option<bool>,
    /// Set to true if you want to get the data from the API instead of the database
    refresh: Option<bool>,
}

impl LuroCommand for Guild {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let mut guild = match self.guild {
            Some(guild_requested) => ctx.get_guild(Id::new(guild_requested.parse()?)).await?,
            None => match &ctx.guild {
                Some(guild) => guild.clone(),
                None => return ctx.simple_response(luro_model::response::SimpleResponse::NotGuild).await,
            },
        };
        let mut luro_guild = String::new();
        let mut guild_description = String::new();
        let mut embed = ctx.default_embed().await;

        if self.refresh.unwrap_or_default() {
            ctx.database.guild_sync(&mut guild).await;
        }

        writeln!(luro_guild, "- Guild Name: {}", &guild.name)?;
        // if !guild.commands.is_empty() {
        //     writeln!(luro_guild, "- Guild Commands: {:#?}", guild.commands)?;
        // }

        writeln!(guild_description, "- Owner: <@{}>", guild.owner_id)?;
        writeln!(guild_description, "- AFK Timeout: {} seconds", guild.afk_timeout.get())?;
        embed
            .title(&guild.name)
            .description(guild_description)
            .create_field("Luro Settings", &luro_guild, false);

        ctx.respond(|response| {
            if self.gdpr_export.unwrap_or_default() {
                if guild.owner_id != ctx.author.user_id {
                    response.content(format!(
                        "Hey <@{}>! <@{}> is being a cunt and trying to steal your guild data!",
                        guild.owner_id, ctx.author.user_id
                    ));
                } else if let Ok(guild_settings) = toml::to_string_pretty(&guild) {
                    response.attachments(
                        vec![Attachment::from_bytes(
                            format!("gdpr-export-{}.txt", ctx.author.user_id),
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
