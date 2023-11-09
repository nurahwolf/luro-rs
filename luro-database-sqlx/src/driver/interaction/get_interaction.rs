use anyhow::Context;
use sqlx::types::Json;
use twilight_model::{application::interaction::{InteractionData, Interaction, InteractionType}, id::{marker::{InteractionMarker, MessageMarker}, Id}, guild::Permissions};
use twilight_model::guild::PartialMember;
use crate::types::DbInteractionKind;

impl crate::SQLxDriver {
    /// Fetches an interaction by message_id
    pub async fn get_interaction_by_message_id(&self, message_id: Id<MessageMarker>) -> anyhow::Result<Interaction> {
        let interaction = sqlx::query_file!(
            "queries/interaction_fetch_by_message_id.sql",
            message_id.get() as i64
        ).fetch_one(&self.pool).await?;

        Ok(Interaction {
            app_permissions: interaction.app_permissions.map(|x|Permissions::from_bits_retain(x as u64)),
            application_id: Id::new(interaction.application_id as u64),
            channel: Some(self.channel_fetch(Id::new(interaction.channel_id as u64)).await?.context("Expected to get channel from DB")?.into()),
            channel_id: Some(Id::new(interaction.channel_id as u64)),
            data: interaction.data.map(|x|x.0),
            guild_id: interaction.guild_id.map(|x| Id::new(x as u64)),
            guild_locale: interaction.guild_locale,
            id: Id::new(interaction.interaction_id as u64),
            kind: match interaction.kind {
                DbInteractionKind::ApplicationCommand => InteractionType::ApplicationCommand,
                DbInteractionKind::ApplicationCommandAutocomplete => InteractionType::ApplicationCommandAutocomplete,
                DbInteractionKind::MessageComponent => InteractionType::MessageComponent,
                DbInteractionKind::ModalSubmit => InteractionType::ModalSubmit,
                DbInteractionKind::Ping => InteractionType::Ping,
                DbInteractionKind::Unknown => return Err(anyhow::anyhow!("Interaction kind unknown returned from database")),
            },
            locale: interaction.locale,
            member: interaction.member.map(|x|x.0),
            message: match interaction.message_id {
                Some(message) => Some(self.get_message(Id::new(message as u64)).await?.context("Should have returned message from database")?.into()),
                None => None,
            },
            token: interaction.token,
            user: self.get_user(Id::new(interaction.user_id as u64)).await?.map(|x|x.into()),
        })
    }

    /// Fetches an interaction by interaction_id
    pub async fn get_interaction(&self, interaction_id: Id<InteractionMarker>) -> anyhow::Result<Interaction> {
        let interaction = sqlx::query_file!(
            "queries/interaction_fetch.sql",
            interaction_id.get() as i64
        ).fetch_one(&self.pool).await?;

        Ok(Interaction {
            app_permissions: interaction.app_permissions.map(|x|Permissions::from_bits_retain(x as u64)),
            application_id: Id::new(interaction.application_id as u64),
            channel: Some(self.channel_fetch(Id::new(interaction.channel_id as u64)).await?.context("Expected to get channel from DB")?.into()),
            channel_id: Some(Id::new(interaction.channel_id as u64)),
            data: interaction.data.map(|x|x.0),
            guild_id: interaction.guild_id.map(|x| Id::new(x as u64)),
            guild_locale: interaction.guild_locale,
            id: Id::new(interaction.interaction_id as u64),
            kind: match interaction.kind {
                DbInteractionKind::ApplicationCommand => InteractionType::ApplicationCommand,
                DbInteractionKind::ApplicationCommandAutocomplete => InteractionType::ApplicationCommandAutocomplete,
                DbInteractionKind::MessageComponent => InteractionType::MessageComponent,
                DbInteractionKind::ModalSubmit => InteractionType::ModalSubmit,
                DbInteractionKind::Ping => InteractionType::Ping,
                DbInteractionKind::Unknown => return Err(anyhow::anyhow!("Interaction kind unknown returned from database")),
            },
            locale: interaction.locale,
            member: interaction.member.map(|x|x.0),
            message: match interaction.message_id {
                Some(message) => Some(self.get_message(Id::new(message as u64)).await?.context("Should have returned message from database")?.into()),
                None => None,
            },
            token: interaction.token,
            user: self.get_user(Id::new(interaction.user_id as u64)).await?.map(|x|x.into()),
        })
    }
}
