use crate::{
    interaction::LuroSlash,
    interactions::moderator::{reason, Reason},
    luro_command::LuroCommand
};
use luro_framework::responses::{StandardResponse, user_action::PunishmentType};
use luro_model::database::drivers::LuroDatabaseDriver;

use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption, ResolvedUser};

#[derive(CommandModel, CreateCommand, Clone, Debug, PartialEq, Eq)]
#[command(name = "fakeban", desc = "Ban a user", dm_permission = false)]
pub struct FakeBan {
    /// The user to ban
    pub user: ResolvedUser,
    /// Message history to purge in seconds. Defaults to 1 day. Max is 604800.
    pub purge: TimeToBan,
    /// The reason they should be banned.
    pub reason: Reason,
    /// Some added description to why they should be banned
    pub details: Option<String>
}

#[derive(CommandOption, CreateOption, Clone, Debug, PartialEq, Eq)]
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
    SevenDays
}

impl LuroCommand for FakeBan {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let interaction = &ctx.interaction;
        let moderator = ctx.get_interaction_author(interaction).await?;
        let punished_user = ctx.framework.database.get_user(&self.user.resolved.id).await?;
        let mut response = ctx.acknowledge_interaction(false).await?;

        let guild_id = interaction.guild_id.unwrap();
        let guild = ctx.framework.twilight_client.guild(guild_id).await?.model().await?;
        let punished_user_id = punished_user.id;
        let reason = reason(self.reason, self.details);
        let period_string = match self.purge {
            TimeToBan::None => "Don't Delete Any".to_string(),
            TimeToBan::Hour => "Previous Hour".to_string(),
            TimeToBan::SixHours => "Previous 6 Hours".to_string(),
            TimeToBan::TwelveHours => "Previous 12 Hours".to_string(),
            TimeToBan::TwentyFourHours => "Previous 24 Hours".to_string(),
            TimeToBan::ThreeDays => "Previous 3 Days".to_string(),
            TimeToBan::SevenDays => "Previous 7 Days".to_string()
        };

        // Checks passed, now let's action the user
        let mut embed =
            StandardResponse::new_punishment(PunishmentType::Banned, &guild.name, &guild.id, &punished_user, &moderator);
        embed
            .punishment_reason(reason.as_deref(), &punished_user)
            .punishment_period(&period_string);
        let punished_user_dm = match ctx.framework.twilight_client.create_private_channel(punished_user_id).await {
            Ok(channel) => channel.model().await?,
            Err(_) => return ctx.respond(|r|r.content("Could not create DM with the user!")).await
        };

        let victim_dm = ctx
            .framework
            .twilight_client
            .create_message(punished_user_dm.id)
            .embeds(&[embed.embed().0])
            .await;

        match victim_dm {
            Ok(_) => embed.create_field("DM Sent", "Successful", true),
            Err(_) => embed.create_field("DM Sent", "Failed", true)
        };

        response.add_embed(embed.embed);
        ctx.send_respond(response).await
    }
}
