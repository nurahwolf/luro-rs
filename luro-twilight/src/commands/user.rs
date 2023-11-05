use luro_framework::{
    CommandInteraction, {CreateLuroCommand, LuroCommand},
};
use twilight_interactions::command::{CommandModel, CreateCommand};

mod identity;

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "user", desc = "User Specific Commands")]
pub enum User {
    #[command(name = "identity")]
    Identity(identity::Identity),
}

impl CreateLuroCommand for User {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        match self {
            Self::Identity(command) => command.interaction_command(ctx).await,
        }
    }

    // async fn interaction_component(self, ctx: ComponentInteraction, _: DatabaseInteraction) -> anyhow::Result<()> {
    //     let user = match self {
    //         Info::User(user_command) => ctx.get_specified_user_or_author(user_command.user.as_ref(), false).await?,
    //         _ => ctx.author.clone(),
    //     };

    //     match ctx.command_name() {
    //         "info-button-guild-permissions" => info_button_guild_permissions(ctx, user).await,
    //         "info-button-messages" => info_recent_messages(ctx, user).await,
    //         name => ctx.response_simple(luro_framework::Response::UnknownCommand(name)).await
    //     }
    // }
}