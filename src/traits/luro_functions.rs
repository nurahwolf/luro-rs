use anyhow::anyhow;
use async_trait::async_trait;
use twilight_interactions::command::ResolvedUser;
use twilight_model::{
    application::interaction::{application_command::InteractionMember, modal::ModalInteractionData, Interaction},
    guild::{Member, PartialMember},
    id::{
        marker::{GuildMarker, UserMarker},
        Id
    },
    user::{CurrentUser, User}
};

use crate::models::LuroFramework;

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

    /// Return a string that is a link to the user's avatar
    fn user_get_avatar(&self, user: &User) -> String {
        let user_id = user.id;

        if let Some(user_avatar) = user.avatar {
            match user_avatar.is_animated() {
                true => return format!("https://cdn.discordapp.com/avatars/{user_id}/{user_avatar}.gif"),
                false => return format!("https://cdn.discordapp.com/avatars/{user_id}/{user_avatar}.png")
            }
        };

        let modulo = user.discriminator % 5;
        format!("https://cdn.discordapp.com/embed/avatars/{modulo}.png")
    }

    /// Return a string that is a link to the member's avatar, falling back to user avatar if it does not exist
    fn member_get_avatar(&self, member: Option<&Member>, guild_id: &Option<Id<GuildMarker>>, user: &User) -> String {
        let user_id = user.id;

        if let Some(member) = member && let Some(guild_id) = guild_id && let Some(member_avatar) = member.avatar {
        match member_avatar.is_animated() {
            true => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.gif"),
            false => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.png"),
        }
    };

        self.user_get_avatar(user)
    }

    /// Return a string that is a link to the user's avatar
    fn currentuser_get_avatar(&self, currentuser: &CurrentUser) -> String {
        let user_id = currentuser.id;

        if let Some(user_avatar) = currentuser.avatar {
            match user_avatar.is_animated() {
                true => return format!("https://cdn.discordapp.com/avatars/{user_id}/{user_avatar}.gif"),
                false => return format!("https://cdn.discordapp.com/avatars/{user_id}/{user_avatar}.png")
            }
        };

        let modulo = currentuser.discriminator % 5;
        format!("https://cdn.discordapp.com/embed/avatars/{modulo}.png")
    }

    /// Return a string that is a link to the user's banner, or [None] if they don't have one
    fn user_get_banner(&self, user: &User) -> Option<String> {
        let user_id = user.id;

        if let Some(banner) = user.banner {
            match banner.is_animated() {
                true => Some(format!("https://cdn.discordapp.com/banner/{user_id}/{banner}.gif")),
                false => Some(format!("https://cdn.discordapp.com/avatars/{user_id}/{banner}.png"))
            }
        } else {
            None
        }
    }

    /// Return a string that is a link to the member's banner, falling back to a user banner if it present. Returns [None] if the user does not have a banner at all.
    fn member_get_banner(&self, _member: &Member, _guild_id: Id<GuildMarker>, user: &User) -> Option<String> {
        let _user_id = user.id;

        // TODO: Looks like this is not possible currently, due to Twilight not having a guild_banner object.

        // if let Some(member) = member && let Some(guild_id) = guild_id && let Some(member_avatar) = member. {
        //     match member_avatar.is_animated() {
        //         true => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.gif"),
        //         false => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.png"),
        //     }
        // };

        self.user_get_banner(user)
    }

    /// Returns the avatar of an [PartialMember]!
    fn partial_member_get_avatar(&self, member: &PartialMember, guild_id: &Option<Id<GuildMarker>>, user: &User) -> String {
        let user_id = user.id;

        if let Some(guild_id) = guild_id && let Some(member_avatar) = member.avatar {
                    match member_avatar.is_animated() {
                        true => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.gif"),
                        false => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.png"),
                    }
                };

        self.user_get_avatar(user)
    }

    /// Get and return useful information about the interaction author
    /// Returns the user, their avatar and a nicely formatted name
    fn get_interaction_author<'a>(&'a self, interaction: &'a Interaction) -> anyhow::Result<(&User, String, &String)> {
        Ok(match interaction.member {
            Some(ref member) => {
                let user = match &member.user {
                    Some(user) => user,
                    None => return Err(anyhow!("Expected user object within member"))
                };
                (
                    user,
                    self.partial_member_get_avatar(member, &interaction.guild_id, user),
                    match &member.nick {
                        Some(nick) => nick,
                        None => &user.name
                    }
                )
            }
            None => match interaction.user {
                Some(ref user) => (user, self.user_get_avatar(user), &user.name),
                None => return Err(anyhow!("No interaction member or user present"))
            }
        })
    }

    /// Get a specified user, else fall back to the interaction author
    /// Returns the user, their avatar and a nicely formatted name
    fn get_specified_user_or_author<'a>(
        &'a self,
        specified_user: &'a Option<ResolvedUser>,
        interaction: &'a Interaction
    ) -> anyhow::Result<(&User, String, &String)> {
        Ok(match specified_user {
            Some(user_defined) => (
                &user_defined.resolved,
                self.interaction_member_get_avatar(&user_defined.member, &interaction.guild_id, &user_defined.resolved),
                match user_defined.member {
                    Some(ref member) => match &member.nick {
                        Some(nick) => nick,
                        None => &user_defined.resolved.name
                    },
                    None => &user_defined.resolved.name
                }
            ),
            None => self.get_interaction_author(interaction)?
        })
    }

    /// Only attempt to get a specific user. If this fails then an error is returned
    /// Returns the user, their avatar and a nicely formatted name
    fn get_specified_user<'a>(&'a self, user: &'a ResolvedUser, interaction: &'a Interaction) -> (&User, String, &String) {
        (
            &user.resolved,
            self.interaction_member_get_avatar(&user.member, &interaction.guild_id, &user.resolved),
            match user.member {
                Some(ref member) => match &member.nick {
                    Some(nick) => nick,
                    None => &user.resolved.name
                },
                None => &user.resolved.name
            }
        )
    }

    /// Returns the avatar of an [InteractionMember]!
    fn interaction_member_get_avatar(
        &self,
        member: &Option<InteractionMember>,
        guild_id: &Option<Id<GuildMarker>>,
        user: &User
    ) -> String {
        let user_id = user.id;

        if let Some(member) = member && let Some(guild_id) = guild_id && let Some(member_avatar) = member.avatar {
                    match member_avatar.is_animated() {
                        true => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.gif"),
                        false => return format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.png"),
                    }
                };

        self.user_get_avatar(user)
    }

    /// Uses the client to explicitly fetch a user. Useful if you only have their ID
    /// Returns the user, their avatar and a nicely formatted name
    async fn fetch_specified_user<'a>(
        &'a self,
        ctx: &'a LuroFramework,
        user_id: &'a Id<UserMarker>
    ) -> anyhow::Result<(User, String, String)> {
        let user = ctx.twilight_client.user(*user_id).await?.model().await?;

        Ok((
            user.clone(),
            self.user_get_avatar(&user),
            if user.discriminator == 0 {
                user.name
            } else {
                format!("{}#{}", user.name, user.discriminator)
            }
        ))
    }
}
