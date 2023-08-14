use anyhow::anyhow;
use twilight_interactions::command::ResolvedUser;
use twilight_model::{application::interaction::Interaction, user::User};

use luro_model::slash_user::SlashUser;

use super::LuroSlash;

impl LuroSlash {
    /// Get and return useful information about the interaction author
    pub fn get_interaction_author<'a>(&'a self, interaction: &'a Interaction) -> anyhow::Result<(&User, SlashUser)> {
        match &interaction.member {
            Some(member) => {
                let user = match &member.user {
                    Some(user) => user,
                    None => return Err(anyhow!("Expected user object within member"))
                };
                Ok((user, SlashUser::from_partialmember(user, member, interaction.guild_id)))
            }
            None => match interaction.user {
                Some(ref user) => Ok((user, SlashUser::from(user))),
                None => Err(anyhow!("No interaction member or user present"))
            }
        }
    }

    /// Get a specified user, else fall back to the interaction author
    /// Returns the user, their avatar and a nicely formatted name
    pub fn get_specified_user_or_author<'a>(
        &'a self,
        specified_user: &'a Option<ResolvedUser>,
        interaction: &'a Interaction
    ) -> anyhow::Result<(&User, SlashUser)> {
        Ok(match specified_user {
            Some(user_defined) => (&user_defined.resolved, SlashUser::from(&user_defined.resolved)),
            None => self.get_interaction_author(interaction)?
        })
    }
}
