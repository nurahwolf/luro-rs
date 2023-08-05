use anyhow::anyhow;
use async_trait::async_trait;
use twilight_interactions::command::ResolvedUser;
use twilight_model::{
    application::interaction::{modal::ModalInteractionData, Interaction},
    user::User
};

use crate::models::SlashUser;

/// A simple trait that implements a bunch of handy features in one place, such as getting a user's avatar. This can be included on other models to make getting date easier.
#[async_trait]
pub trait LuroFunctions {
    /// Parse a field from [`ModalInteractionData`].
    ///
    /// This function try to find a field with the given name in the modal data and
    /// return its value as a string.
    fn parse_modal_field<'a>(&self, data: &'a ModalInteractionData, name: &str) -> Result<Option<&'a str>, anyhow::Error> {
        let mut components = data.components.iter().flat_map(|c| &c.components);

        match components.find(|c| &*c.custom_id == name) {
            Some(component) => Ok(component.value.as_deref()),
            None => Err(anyhow!("missing modal field: {}", name))
        }
    }

    /// Parse a required field from [`ModalInteractionData`].
    ///
    /// This function is the same as [`parse_modal_field`] but returns an error if
    /// the field value is [`None`].
    fn parse_modal_field_required<'a>(&self, data: &'a ModalInteractionData, name: &str) -> Result<&'a str, anyhow::Error> {
        let value = self.parse_modal_field(data, name)?;

        value.ok_or_else(|| anyhow!("required modal field is empty: {}", name))
    }

    /// Get and return useful information about the interaction author
    fn get_interaction_author<'a>(&'a self, interaction: &'a Interaction) -> anyhow::Result<(&User, SlashUser)> {
        match &interaction.member {
            Some(member) => {
                let user = match &member.user {
                    Some(user) => user,
                    None => return Err(anyhow!("Expected user object within member"))
                };
                Ok((user, SlashUser::from_member(user, member.avatar, interaction.guild_id)))
            }
            None => match interaction.user {
                Some(ref user) => Ok((user, SlashUser::from(user))),
                None => Err(anyhow!("No interaction member or user present"))
            }
        }
    }

    /// Get a specified user, else fall back to the interaction author
    /// Returns the user, their avatar and a nicely formatted name
    fn get_specified_user_or_author<'a>(
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
