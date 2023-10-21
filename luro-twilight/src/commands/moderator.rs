use async_trait::async_trait;
use luro_builder::embed::EmbedBuilder;
use luro_framework::{
    LuroInteraction, command::LuroCommandTrait, CommandInteraction, ModalInteraction,
};
use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption};
use twilight_model::id::{marker::UserMarker, Id};

use luro_model::{
    database_driver::LuroDatabaseDriver,
    guild::log_channel::LuroLogChannel,
    user::{actions::UserActions, actions_type::UserActionType},
};

mod assign;
mod ban;
mod kick;
mod modify;
mod purge;
mod settings;
mod sync;
mod unban;
mod warn;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "mod", desc = "Commands that can be used by moderators", dm_permission = false)]
pub enum Moderator {
    #[command(name = "ban")]
    Ban(ban::Ban),
    #[command(name = "kick")]
    Kick(kick::Kick),
    #[command(name = "purge")]
    Purge(purge::Purge),
    #[command(name = "settings")]
    Setting(settings::Settings),
    #[command(name = "warn")]
    Warn(warn::Warn),
    #[command(name = "unban")]
    Unban(unban::Unban),
    #[command(name = "sync")]
    Sync(sync::Sync),
    #[command(name = "modify")]
    Modify(modify::Modify),
}

#[async_trait]
impl LuroCommandTrait for Moderator {
    async fn handle_interaction(
        ctx: CommandInteraction<Self>,
    ) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        match ctx.command {
            Self::Ban(_) => ban::Ban::handle_interaction(ctx).await,
            Self::Kick(_) => kick::Kick::handle_interaction(ctx).await,
            Self::Purge(_) => purge::Purge::handle_interaction(ctx).await,
            Self::Setting(_) => settings::Settings::handle_interaction(ctx).await,
            Self::Warn(_) => warn::Warn::handle_interaction(ctx).await,
            Self::Unban(_) => unban::Unban::handle_interaction(ctx).await,
            Self::Sync(_) => sync::Sync::handle_interaction(ctx).await,
            Self::Modify(_) => modify::Modify::handle_interaction(ctx).await,
        }
    }

    async fn handle_modal(ctx: ModalInteraction<Self>) -> anyhow::Result<()> {
        let author = ctx.author();
        let warning = ctx.parse_field_required("mod-warn-text")?;
        let id = ctx.parse_field_required("mod-warn-id")?;
        let user_id: Id<UserMarker> = Id::new(id.parse::<u64>()?);

        let luro_user = ctx.database.get_user(&ctx.author.user_id()).await?;

        let mut user_data = ctx.database.get_user(&user_id).await?;
        user_data.warnings.push((warning.to_owned(), author.id));
        ctx.database.modify_user(&user_id, &user_data).await?;

        let mut embed = EmbedBuilder::default();
        embed
            .description(format!("Warning Created for <@{user_id}>\n```{warning}```"))
            .colour(ctx.accent_colour(&ctx).await)
            .footer(|footer| footer.text(format!("User has a total of {} warnings.", user_data.warnings.len())))
            .author(|author| {
                author
                    .name(format!("Warning by {}", luro_user.name()))
                    .icon_url(luro_user.avatar())
            });

        match ctx.twilight_client.create_private_channel(user_id).await {
            Ok(channel) => {
                let channel = channel.model().await?;
                let victim_dm = ctx
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

        ctx.send_log_channel(&ctx.guild_id.unwrap(), LuroLogChannel::Moderator, |r| {
            r.add_embed(embed.clone())
        })
        .await?;

        let mut reward = ctx.database.get_user(&author.id).await?;
        reward.moderation_actions_performed += 1;
        ctx.database.modify_user(&author.id, &reward).await?;

        // Record the punishment
        let mut warned = ctx.database.get_user(&user_id).await?;
        warned.moderation_actions.push(UserActions {
            action_type: vec![UserActionType::Warn],
            guild_id: ctx.guild_id,
            reason: Some(warning.to_owned()),
            responsible_user: author.id,
        });
        ctx.database.modify_user(&user_id, &warned).await?;

        ctx.respond(|response| response.add_embed(embed)).await
    }
}

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
