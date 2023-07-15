/// Implement `handle` method for a guild command type that is only available in
/// guilds.
///
/// This macro is identical to [`impl_command_handle`] except that it will use
/// [`GuildInteractionContext`] instead of [`InteractionContext`].
///
/// The command type must implement [`CommandModel`] and have an `exec` method
/// with the following signature:
///
///`async fn exec(self, ctx: GuildInteractionContext, state: &ClusterState) -> Result<InteractionResponse, anyhow::Error>`
#[macro_export]
macro_rules! impl_guild_command_handle {
    ($name:path) => {
        impl $name {
            #[::tracing::instrument]
            pub async fn handle(
                mut interaction: ::twilight_model::application::interaction::Interaction,
                state: &$crate::cluster::ClusterState,
            ) -> Result<$crate::interaction::response::InteractionResponse, ::anyhow::Error> {
                let parsed =
                    $crate::interaction::util::parse_command_data::<Self>(&mut interaction)?;
                let ctx = $crate::interaction::util::GuildInteractionContext::new(interaction)?;

                parsed.exec(ctx, state).await
            }
        }
    };
}
