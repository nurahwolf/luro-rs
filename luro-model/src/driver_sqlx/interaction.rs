use sqlx::Error;
use twilight_model::{application::interaction::Interaction, id::Id};

use super::PostgresDriver;
pub struct DatabaseInteraction {
    pub application_id: i64,
    pub interaction_id: i64,
    pub message_id: Option<i64>,
    pub data: Vec<u8>,
    pub kind: Vec<u8>,
    pub token: String,
}

impl TryFrom<Interaction> for DatabaseInteraction {
    type Error = anyhow::Error;

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
    type Error = anyhow::Error;

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

// impl From<Interaction> for DatabaseInteraction {
//     fn from(interaction: Interaction) -> Self {
//         Self {
//             interaction_id: interaction.id.get() as i64,
//             message_id: interaction.message.map(|x| x.id.get() as i64),
//             data: bincode::serialize(&interaction.data.unwrap()).unwrap().to_vec(),
//         }
//     }
// }

impl PostgresDriver {
    /// Fetches an interaction by interaction_id
    pub async fn get_interaction(&self, id: i64) -> Result<Option<DatabaseInteraction>, Error> {
        let query = sqlx::query_as!(
            DatabaseInteraction,
            "SELECT * FROM interactions WHERE interaction_id = $1",
            id
        );

        let data = query.fetch_optional(&self.0).await?;

        Ok(data)
    }

    /// Fetches an interaction by message_id
    pub async fn get_interaction_by_message_id(&self, id: i64) -> Result<Option<DatabaseInteraction>, Error> {
        let query = sqlx::query_as!(DatabaseInteraction, "SELECT * FROM interactions WHERE message_id = $1", id);

        let data = query.fetch_optional(&self.0).await?;

        Ok(data)
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
