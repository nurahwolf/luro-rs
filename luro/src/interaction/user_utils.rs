use luro_model::{database::drivers::LuroDatabaseDriver, user::LuroUser};
use twilight_interactions::command::ResolvedUser;
use twilight_model::application::interaction::Interaction;

use super::LuroSlash;

impl<D: LuroDatabaseDriver> LuroSlash<D> {
    /// Get and return useful information about the interaction author
    pub async fn get_interaction_author<'a>(&'a self, interaction: &'a Interaction) -> anyhow::Result<LuroUser> {
        self.framework.database.get_user(&interaction.author_id().unwrap(), &self.framework.twilight_client).await
    }

    /// Get a specified user, else fall back to the interaction author
    /// Returns the user, their avatar and a nicely formatted name
    pub async fn get_specified_user_or_author<'a>(
        &'a self,
        specified_user: &'a Option<ResolvedUser>,
        interaction: &'a Interaction
    ) -> anyhow::Result<LuroUser> {
        match specified_user {
            Some(user_defined) => self.framework.database.get_user(&user_defined.resolved.id, &self.framework.twilight_client).await,
            None => self.get_interaction_author(interaction).await
        }
    }
}
