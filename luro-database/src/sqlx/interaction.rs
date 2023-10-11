use twilight_model::{application::interaction::Interaction, id::Id};

use crate::{DatabaseInteraction, LuroDatabase};

impl TryFrom<Interaction> for DatabaseInteraction {
    type Error = bincode::Error;

    fn try_from(interaction: Interaction) -> Result<Self, Self::Error> {
        Ok(Self {
            interaction_id: interaction.id.get() as i64,
            message_id: interaction.message.map(|x| x.id.get() as i64),
            data: bincode::serialize(&interaction.data.unwrap())?.to_vec(),
            application_id: interaction.application_id.get() as i64,
            kind: bincode::serialize(&interaction.kind)?.to_vec(),
            token: interaction.token,
        })
    }
}

impl TryInto<Interaction> for DatabaseInteraction {
    type Error = bincode::Error;

    // TODO: Remove this when possible
    #[allow(deprecated)]
    fn try_into(self) -> Result<Interaction, Self::Error> {
        Ok(Interaction {
            app_permissions: None,
            application_id: Id::new(self.application_id as u64),
            channel: None,
            channel_id: None,
            data: bincode::deserialize(&self.data)?,
            guild_id: None,
            guild_locale: None,
            id: Id::new(self.interaction_id as u64),
            kind: bincode::deserialize(&self.kind)?,
            locale: None,
            member: None,
            message: None,
            token: self.token,
            user: None,
        })
    }
}

impl LuroDatabase {
        pub async fn count_interactions(&self) -> Result<i64, sqlx::Error> {
            let query = sqlx::query!("
            SELECT 
                COUNT(*) as count
            FROM 
                interactions
            ").fetch_all(&self.0).await?;
    
            let result = query.into_iter().map(|x|x.count.unwrap_or_default()).collect::<Vec<_>>();
            Ok(result.first().copied().unwrap_or_default())
        }

    /// Fetches an interaction by interaction_id
    pub async fn get_interaction(&self, id: i64) -> Result<Option<DatabaseInteraction>, sqlx::Error> {
        let query = sqlx::query_as!(
            DatabaseInteraction,
            "SELECT * FROM interactions WHERE interaction_id = $1",
            id
        );

        query.fetch_optional(&self.0).await
    }

    /// Fetches an interaction by message_id
    pub async fn get_interaction_by_message_id(&self, id: i64) -> Result<Option<DatabaseInteraction>, sqlx::Error> {
        let query = sqlx::query_as!(DatabaseInteraction, "SELECT * FROM interactions WHERE message_id = $1", id);

        query.fetch_optional(&self.0).await
    }

    pub async fn update_interaction(&self, interaction: Interaction) -> anyhow::Result<DatabaseInteraction> {
        //  let authorities = unsafe { Authorities::deserialize(&authorities) };
        let interaction = DatabaseInteraction::try_from(interaction)?;
        let query = sqlx::query_as!(
            DatabaseInteraction,
            "INSERT INTO interactions (interaction_id, message_id, data, application_id, token, kind)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (interaction_id)
            DO UPDATE SET message_id = $2, data = $3
            RETURNING interaction_id, message_id, data, application_id, token, kind",
            interaction.interaction_id,
            interaction.message_id,
            interaction.data,
            interaction.application_id,
            interaction.token,
            interaction.kind,
        );

        Ok(query.fetch_one(&self.0).await?)
    }
}
