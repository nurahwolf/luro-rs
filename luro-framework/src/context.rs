use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::application::interaction::message_component::MessageComponentInteractionData;
use twilight_model::application::interaction::modal::ModalInteractionData;

use crate::InteractionModal;
use crate::{InteractionCommand, InteractionComponent, InteractionContext};

pub mod parse_modal_field;

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
