use anyhow::Context;
use sqlx::types::Json;
use twilight_model::application::interaction::Interaction;

use crate::types::DbInteractionKind;

impl crate::SQLxDriver {
    pub async fn update_interaction(&self, interaction: &Interaction) -> anyhow::Result<u64> {
        Ok(sqlx::query_file!(
            "queries/interaction_update.sql",
            interaction.app_permissions.map(|x|x.bits() as i64),
            interaction.application_id.get() as i64,
            interaction.channel.as_ref().map(|x|x.id.get() as i64),
            interaction.data.as_ref().map(Json) as _,
            interaction.guild_id.map(|x|x.get() as i64),
            interaction.guild_locale,
            interaction.id.get() as i64,
            match interaction.kind {
                twilight_model::application::interaction::InteractionType::Ping => DbInteractionKind::Ping,
                twilight_model::application::interaction::InteractionType::ApplicationCommand => DbInteractionKind::ApplicationCommand,
                twilight_model::application::interaction::InteractionType::MessageComponent => DbInteractionKind::MessageComponent,
                twilight_model::application::interaction::InteractionType::ApplicationCommandAutocomplete => DbInteractionKind::ApplicationCommandAutocomplete,
                twilight_model::application::interaction::InteractionType::ModalSubmit => DbInteractionKind::ModalSubmit,
                data => return Err(anyhow::anyhow!("Trying to sync unexpected interaction kind with the database: {data:#?}")),
            } as _,
            interaction.locale,
            interaction.message.as_ref().map(|x|x.id.get() as i64),
            interaction.token,
            match interaction.member {
                Some(ref member) => member.user.as_ref().context("Expected partial member to contain user")?.id,
                None => interaction.user.as_ref().context("No member, so expected to get user data")?.id,
            }.get() as i64,
        ).execute(&self.pool).await.map(|x|x.rows_affected())?)
    }
}
