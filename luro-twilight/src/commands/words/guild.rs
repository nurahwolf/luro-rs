use luro_framework::{CommandInteraction, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::UserMarker, Id};

#[derive(CommandModel, CreateCommand)]
#[command(name = "guild", desc = "Get some stats on the bullshit someone has posted.")]
pub struct Guild {
    /// The user to get the stats of
    user: Option<Id<UserMarker>>,
    /// How many words we should get stats for. Defaults to 10.
    limit: Option<i64>,
    /// A particular word to search word
    word: Option<String>,
    /// Search across ALL user data for word stats. This can be very slow!
    global: Option<bool>,
}

impl LuroCommand for Guild {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        ctx.respond(|r| r.content("Not yet implemented!").ephemeral()).await
    }
}
