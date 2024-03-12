use twilight_model::{
    application::interaction::Interaction,
    id::{marker::InteractionMarker, Id},
};

use crate::models::interaction::{InteractionError, InteractionResult};

use super::Database;

impl Database {
    /// Fetches an interaction by interaction_id
    pub async fn fetch_interaction(
        &self,
        id: Id<InteractionMarker>,
    ) -> InteractionResult<Interaction>{
        #[cfg(feature = "database-sqlx")]
        match fetch_interaction_sqlx(self, id).await {
            Ok(Some(interaction)) => return Ok(interaction),
            Ok(None) => {
                tracing::debug!("Interaction `{id}` was not found in the database.")
            }
            Err(why) => {
                tracing::error!("Error raised while trying to find interaction `{id}`: {why}")
            }
        };

        Err(InteractionError::RequiresDatabase)
    }
}

#[cfg(feature = "database-sqlx")]
#[derive(Debug, ::sqlx::Type)]
#[sqlx(type_name = "interaction_kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DbInteractionKind {
    ApplicationCommand,
    ApplicationCommandAutocomplete,
    MessageComponent,
    ModalSubmit,
    Ping,
    Unknown,
}

#[cfg(feature = "database-sqlx")]
pub async fn fetch_interaction_sqlx(
    db: &Database,
    id: Id<InteractionMarker>,
) -> InteractionResult<Option<Interaction>> {
    use sqlx::types::Json;
    use twilight_model::application::interaction::InteractionData;
    use twilight_model::guild::PartialMember;

    let interaction = sqlx::query_file!(
        "src/database/sqlx_queries/interaction/interaction_fetch.sql",
        id.get() as i64
    )
    .fetch_optional(&db.pool)
    .await?;

    let interaction = match interaction {
        Some(interaction) => interaction,
        None => return Ok(None),
    };

    #[allow(deprecated)]
    Ok(Some(Interaction {
        app_permissions: interaction.app_permissions.map(|x| twilight_model::guild::Permissions::from_bits_retain(x as u64)),
        application_id: twilight_model::id::Id::new(interaction.application_id as u64),
        channel: Some(
            db.fetch_channel(Id::new(interaction.channel_id as u64))
                .await?
        ),
        channel_id: Some(Id::new(interaction.channel_id as u64)),
        data: interaction.data.map(|x| x.0),
        guild_id: interaction.guild_id.map(|x| twilight_model::id::Id::new(x as u64)),
        guild_locale: interaction.guild_locale,
        id: twilight_model::id::Id::new(interaction.interaction_id as u64),
        kind: match interaction.kind {
            DbInteractionKind::ApplicationCommand => twilight_model::application::interaction::InteractionType::ApplicationCommand,
            DbInteractionKind::ApplicationCommandAutocomplete => twilight_model::application::interaction::InteractionType::ApplicationCommandAutocomplete,
            DbInteractionKind::MessageComponent => twilight_model::application::interaction::InteractionType::MessageComponent,
            DbInteractionKind::ModalSubmit => twilight_model::application::interaction::InteractionType::ModalSubmit,
            DbInteractionKind::Ping => twilight_model::application::interaction::InteractionType::Ping,
            DbInteractionKind::Unknown => twilight_model::application::interaction::InteractionType::Ping, // TODO: Changeme
        },
        locale: interaction.locale,
        member: interaction.member.map(|x| x.0),
        message: match interaction.message_id {
            Some(message) => Some(
                // TODO: Changeme
                db.fetch_message(Id::new(interaction.channel_id as u64),Id::new(message as u64))
                    .await?
                    .into(),
            ),
            None => None,
        },
        token: interaction.token,
        user: Some(db.fetch_user(Id::new(interaction.user_id as u64)).await?.into()),
    }))
}
