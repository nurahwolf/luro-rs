use twilight_interactions::command::ResolvedUser;
use twilight_model::user::User;

use crate::models::{LuroResponse, SlashUser};

use super::LuroFramework;

impl LuroFramework {
    /// Get a specified user, else fall back to the interaction author
    /// Returns the user, their avatar and a nicely formatted name
    pub fn get_specified_user_or_author<'a>(
        &'a self,
        specified_user: &'a Option<ResolvedUser>,
        slash: &'a LuroResponse
    ) -> anyhow::Result<(&User, SlashUser)> {
        Ok(match specified_user {
            Some(user_defined) => (&user_defined.resolved, SlashUser::from(&user_defined.resolved)),
            None => self.get_interaction_author(slash)?
        })
    }
}
