use tracing::info;
use twilight_gateway::{Event, Latency, MessageSender};
use twilight_http::client::InteractionClient;
use twilight_model::application::command::Command;
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::application::interaction::message_component::MessageComponentInteractionData;
use twilight_model::application::interaction::modal::ModalInteractionData;

use crate::{Context, Framework, InteractionModal};
use crate::{InteractionCommand, InteractionComponent, InteractionContext};

impl Context {
    pub fn new(framework: Framework, event: Event, latency: Latency, shard: MessageSender) -> Self {
        Self {
            cache: framework.cache,
            database: framework.database,
            event,
            global_commands: framework.global_commands,
            guild_commands: framework.guild_commands,
            http_client: framework.http_client,
            latency,
            lavalink: framework.lavalink,
            shard,
            tracing_subscriber: framework.tracing_subscriber,
            twilight_client: framework.twilight_client,
        }
    }

    /// Gets the [interaction client](InteractionClient) using this framework's
    /// [http client](Client) and [application id](ApplicationMarker)
    pub fn interaction_client(&self) -> InteractionClient {
        self.twilight_client.interaction(self.database.application.read().unwrap().id)
    }

    /// Register commands to the Discord API.
    pub async fn register_commands(&self, commands: &[Command]) -> anyhow::Result<()> {
        let client = self.interaction_client();

        match client.set_global_commands(commands).await {
            Ok(command_result) => Ok(info!(
                "Successfully registered {} global commands!",
                command_result.model().await?.len()
            )),
            Err(why) => Err(why.into()),
        }
    }
}

impl InteractionCommand {
    pub fn new(interaction: InteractionContext, data: Box<CommandData>) -> Self {
        Self {
            application_id: interaction.application_id,
            channel: interaction.channel.unwrap(),
            data,
            guild_id: interaction.guild_id,
            id: interaction.id,
            latency: interaction.latency,
            member: interaction.member,
            permissions: interaction.app_permissions,
            shard: interaction.shard,
            token: interaction.token,
            user: interaction.user,
            original: interaction.original,
        }
    }
}

impl InteractionComponent {
    pub fn new(interaction: InteractionContext, data: Box<MessageComponentInteractionData>) -> Self {
        Self {
            original: interaction.original,
            application_id: interaction.application_id,
            channel: interaction.channel.unwrap(),
            data,
            guild_id: interaction.guild_id,
            id: interaction.id,
            latency: interaction.latency,
            member: interaction.member,
            message: interaction.message.unwrap(),
            permissions: interaction.app_permissions,
            shard: interaction.shard,
            token: interaction.token,
            user: interaction.user,
        }
    }
}

impl InteractionModal {
    pub fn new(interaction: InteractionContext, data: ModalInteractionData) -> Self {
        Self {
            application_id: interaction.application_id,
            channel: interaction.channel.unwrap(),
            data,
            guild_id: interaction.guild_id,
            id: interaction.id,
            latency: interaction.latency,
            member: interaction.member,
            message: interaction.message,
            permissions: interaction.app_permissions,
            shard: interaction.shard,
            token: interaction.token,
            user: interaction.user,
            original: interaction.original,
        }
    }
}
