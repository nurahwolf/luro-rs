use anyhow::anyhow;
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
            channel: interaction.channel,
            data,
            guild_id: interaction.guild_id,
            id: interaction.id,
            latency: interaction.latency,
            member: interaction.member,
            permissions: interaction.app_permissions,
            shard: interaction.shard,
            token: interaction.token,
            user: interaction.user
        }
    }
}

impl InteractionComponent {
    pub fn new(interaction: InteractionContext, data: Box<MessageComponentInteractionData>) -> anyhow::Result<Self> {
        match interaction.message {
            Some(message) => Ok(Self {
                original: interaction.original,
                application_id: interaction.application_id,
                channel: interaction.channel,
                data,
                guild_id: interaction.guild_id,
                id: interaction.id,
                latency: interaction.latency,
                member: interaction.member,
                message,
                permissions: interaction.app_permissions,
                shard: interaction.shard,
                token: interaction.token,
                user: interaction.user
            }),
            None => Err(anyhow!("No message found!"))
        }
    }
}

impl InteractionModal {
    pub fn new(interaction: InteractionContext, data: ModalInteractionData) -> Self {
        Self {
            application_id: interaction.application_id,
            channel: interaction.channel,
            data,
            guild_id: interaction.guild_id,
            id: interaction.id,
            latency: interaction.latency,
            member: interaction.member,
            message: interaction.message,
            permissions: interaction.app_permissions,
            shard: interaction.shard,
            token: interaction.token,
            user: interaction.user
        }
    }
}
