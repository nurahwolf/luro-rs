use luro_framework::{CommandInteraction, LuroCommand};
use luro_model::response::SimpleResponse;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::Id;

#[derive(CommandModel, CreateCommand)]
#[command(name = "guild", desc = "Stats for a guild")]
pub struct Guild {
    /// The guild to get the stats of
    guild: Option<i64>,
    /// How many words we should get stats for. Defaults to 10.
    _limit: Option<i64>,
    /// A particular word to search word
    _word: Option<String>,
}

impl LuroCommand for Guild {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let _guild = match self.guild {
            Some(guild_id) => ctx.database.guild_fetch(Id::new(guild_id as u64)).await?,
            None => match &ctx.guild {
                Some(guild) => guild.clone(),
                None => return ctx.simple_response(SimpleResponse::NotGuild).await,
            },
        };

        ctx.respond(|r| r.content("Not yet implemented!").ephemeral()).await
    }
}
