use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption};
use twilight_model::id::{marker::UserMarker, Id};

use crate::{
    embeds::{dm_sent, user_banned},
    models::interaction::{InteractionContext, InteractionResult},
};

#[derive(CommandOption, CreateOption, Debug, PartialEq)]
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
    #[option(
        name = "Raider - Someone who joined just to be a little bitch",
        value = "raider"
    )]
    Raider,

    /// Racist, Sexist and other such things.
    #[option(
        name = "Vile - Racist, Sexist and other such plesent things.",
        value = ""
    )]
    Vile,

    /// A completely custom reason if the others do not fit
    #[option(
        name = "Custom Reason - A completely custom reason if the others do not fit",
        value = "custom"
    )]
    Custom,
}

#[derive(CommandOption, CreateOption)]
pub enum TimeToBan {
    #[option(name = "Don't Delete Any", value = 0)]
    None,
    #[option(name = "Previous Hour", value = 3_600)]
    Hour,
    #[option(name = "Previous 6 Hours", value = 21_600)]
    SixHours,
    #[option(name = "Previous 12 Hours", value = 43_200)]
    TwelveHours,
    #[option(name = "Previous 24 Hours", value = 86_400)]
    TwentyFourHours,
    #[option(name = "Previous 3 Days", value = 259_200)]
    ThreeDays,
    #[option(name = "Previous 7 Days", value = 604_800)]
    SevenDays,
}

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "fakeban",
    desc = "Ban a user (not really)",
    dm_permission = false
)]
pub struct Fakeban {
    /// The user to ban
    pub user_id: Id<UserMarker>,
    /// Message history to purge in seconds. Defaults to 1 day. Max is 604800.
    pub purge: TimeToBan,
    /// The reason they should be banned.
    pub reason: Reason,
    /// Some added description to why they should be banned
    pub details: Option<String>,
}

impl crate::models::CreateCommand for Fakeban {
    async fn handle_command(self, ctx: &mut InteractionContext) -> InteractionResult<()> {
        ctx.ack_interaction(false).await?;

        let twilight_client = &ctx.gateway.twilight_client;
        let guild = ctx.guild().await?;
        let author = ctx.author_member(guild.id()).await?;
        let target = ctx.fetch_user(self.user_id).await?;

        let reason = reason(self.reason, self.details);

        let mut embed = user_banned(
            &target,
            &author,
            reason.as_deref(),
            self.purge.value(),
            &guild.twilight_guild.name,
        )?;

        let target_dm = twilight_client
            .create_private_channel(target.user_id())
            .await;

        let dm_success = match target_dm {
            Ok(channel) => {
                let target_dm = channel.model().await?;
                twilight_client
                    .create_message(target_dm.id)
                    .embeds(&[embed.0.clone()])
                    .await
                    .is_ok()
            }
            Err(_) => false,
        };

        // Modify the original embed
        dm_sent(&mut embed, dm_success);

        ctx.respond(|r| r.add_embed(embed)).await
    }
}

fn reason(reason: Reason, details: Option<String>) -> Option<String> {
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
