use crate::CommandInteraction;

impl<T> CommandInteraction<T> {
    /// Overwrite the command data
    pub fn command_data<N>(self, command: N) -> CommandInteraction<N> {
        CommandInteraction {
            command,
            app_permissions: self.app_permissions,
            application_id: self.application_id,
            cache: self.cache,
            channel: self.channel,
            data: self.data,
            database: self.database,
            global_commands: self.global_commands,
            guild_commands: self.guild_commands,
            guild_id: self.guild_id,
            guild_locale: self.guild_locale,
            http_client: self.http_client,
            id: self.id,
            kind: self.kind,
            latency: self.latency,
            #[cfg(feature = "lavalink")]
            lavalink: self.lavalink,
            locale: self.locale,
            member: self.member,
            message: self.message,
            original: self.original,
            shard: self.shard,
            token: self.token,
            tracing_subscriber: self.tracing_subscriber,
            twilight_client: self.twilight_client,
            user: self.user,
        }
    }
}
