use anyhow::anyhow;
use twilight_model::user::User;

use crate::models::{LuroResponse, SlashUser};

use super::LuroFramework;

impl LuroFramework {
    /// Get and return useful information about the interaction author
    pub fn get_interaction_author<'a>(&'a self, slash: &'a LuroResponse) -> anyhow::Result<(&'a User, SlashUser)> {
        match &slash.interaction.member {
            Some(member) => {
                let user = match &member.user {
                    Some(user) => user,
                    None => return Err(anyhow!("Expected user object within member"))
                };
                Ok((user, SlashUser::from_partialmember(user, member, slash.interaction.guild_id)))
            }
            None => match &slash.interaction.user {
                Some(user) => Ok((user, SlashUser::from(user))),
                None => Err(anyhow!("No interaction member or user present"))
            }
        }
    }
}
