use anyhow::Context;
use luro_builder::embed::EmbedBuilder;
use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption};
use twilight_model::{
    application::interaction::modal::ModalInteractionData,
    id::{marker::UserMarker, Id},
};

use crate::interaction::LuroSlash;
use luro_model::{
    database::drivers::LuroDatabaseDriver,
    guild::log_channel::LuroLogChannel,
    user::{actions::UserActions, actions_type::UserActionType},
};

use self::{
    ban::Ban, kick::Kick, purge::PurgeCommand, settings::GuildSettingsCommand, sync::SyncCommand, unban::Unban,
    warn::ModeratorWarnCommand,
};
use crate::luro_command::LuroCommand;

mod assign;
mod ban;
mod kick;
mod purge;
mod settings;
mod sync;
mod unban;
mod warn;

#[derive(CommandOption, CreateOption, Clone, Debug, PartialEq, Eq)]
pub enum Reason {
    /// Someone who attempts to steal your money by offering fake commissions
    #[option(
        name = "Art Scam - Someone who attempts to steal your money by offering fake commissions",
        value = "art-scam"
    )]
    ArtScam,

    /// Compromised Account
    #[option(
        name = "Compromised Account - An account that has been token logged, or is spreading malware",
        value = "compromised"
    )]
    Compromised,

    /// Someone who is being a little bitch
    #[option(name = "Troll - Someone who is being a little bitch", value = "troll")]
    Troll,

    /// Someone who joined just to be a little bitch
    #[option(name = "Raider - Someone who joined just to be a little bitch", value = "raider")]
    Raider,

    /// Racist, Sexist and other such things.
    #[option(name = "Vile - Racist, Sexist and other such plesent things.", value = "")]
    Vile,

    /// A completely custom reason if the others do not fit
    #[option(
        name = "Custom Reason - A completely custom reason if the others do not fit",
        value = "custom"
    )]
    Custom,
}

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "mod", desc = "Commands that can be used by moderators", dm_permission = false)]
pub enum ModeratorCommands {
    #[command(name = "ban")]
    Ban(Ban),
    #[command(name = "kick")]
    Kick(Kick),
    #[command(name = "purge")]
    Purge(PurgeCommand),
    #[command(name = "settings")]
    Setting(GuildSettingsCommand),
    #[command(name = "warn")]
    Warn(ModeratorWarnCommand),
    #[command(name = "unban")]
    Unban(Unban),
    #[command(name = "sync")]
    Sync(SyncCommand),
}

impl LuroCommand for ModeratorCommands {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        match self {
            Self::Ban(command) => command.run_command(ctx).await,
            Self::Kick(command) => command.run_command(ctx).await,
            Self::Purge(command) => command.run_command(ctx).await,
            Self::Setting(command) => command.run_command(ctx).await,
            Self::Warn(command) => command.run_command(ctx).await,
            Self::Unban(command) => command.run_command(ctx).await,
            Self::Sync(command) => command.run_command(ctx).await,
        }
    }

    async fn handle_model<D: LuroDatabaseDriver>(data: ModalInteractionData, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let author = ctx.interaction.author().context("Expected to get interaction author")?;
        let warning = ctx.parse_modal_field_required(&data, "mod-warn-text")?;
        let id = ctx.parse_modal_field_required(&data, "mod-warn-id")?;
        let user_id: Id<UserMarker> = Id::new(id.parse::<u64>()?);

        let luro_user = ctx.framework.database.get_user(&ctx.interaction.author_id().unwrap()).await?;

        let mut user_data = ctx.framework.database.get_user(&user_id).await?;
        user_data.warnings.push((warning.to_owned(), author.id));
        ctx.framework.database.save_user(&user_id, &user_data).await?;

        let mut embed = EmbedBuilder::default();
        embed
            .description(format!("Warning Created for <@{user_id}>\n```{warning}```"))
            .colour(ctx.accent_colour().await)
            .footer(|footer| footer.text(format!("User has a total of {} warnings.", user_data.warnings.len())))
            .author(|author| {
                author
                    .name(format!("Warning by {}", luro_user.name()))
                    .icon_url(luro_user.avatar())
            });

        match ctx.framework.twilight_client.create_private_channel(user_id).await {
            Ok(channel) => {
                let channel = channel.model().await?;
                let victim_dm = ctx
                    .framework
                    .twilight_client
                    .create_message(channel.id)
                    .embeds(&[embed.clone().into()])
                    .await;
                match victim_dm {
                    Ok(_) => embed.create_field("DM Sent", "Successful", true),
                    Err(_) => embed.create_field("DM Sent", "Failed", true),
                }
            }
            Err(_) => embed.create_field("DM Sent", "Failed", true),
        };

        ctx.send_log_channel(LuroLogChannel::Moderator, |r| r.add_embed(embed.clone()))
            .await?;

        let mut reward = ctx.framework.database.get_user(&author.id).await?;
        reward.moderation_actions_performed += 1;
        ctx.framework.database.save_user(&author.id, &reward).await?;

        // Record the punishment
        let mut warned = ctx.framework.database.get_user(&user_id).await?;
        warned.moderation_actions.push(UserActions {
            action_type: vec![UserActionType::Warn],
            guild_id: ctx.interaction.guild_id,
            reason: Some(warning.to_owned()),
            responsible_user: author.id,
        });
        ctx.framework.database.save_user(&user_id, &warned).await?;

        ctx.respond(|response| response.add_embed(embed)).await
    }
}

pub fn reason(reason: Reason, details: Option<String>) -> Option<String> {
    let mut reason_string = match reason {
        Reason::ArtScam => "[Art Scam]".to_owned(),
        Reason::Compromised => "[Compromised Account]".to_owned(),
        Reason::Custom => String::new(),
        Reason::Raider => "[Raider]".to_owned(),
        Reason::Troll => "[Troll]".to_owned(),
        Reason::Vile => "[Vile]".to_owned(),
    };

    if let Some(details) = details {
        match reason == Reason::Custom {
            true => reason_string.push_str(&details.to_string()),
            false => reason_string.push_str(&format!(" - {details}")),
        }
    }

    match reason_string.is_empty() {
        true => None,
        false => Some(reason_string),
    }
}
