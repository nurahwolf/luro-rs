use luro_builder::embed::EmbedBuilder;
use luro_model::{database::drivers::LuroDatabaseDriver, response::LuroResponse, user::LuroUser, ACCENT_COLOUR};
use tracing::warn;
use twilight_interactions::command::ResolvedUser;
use twilight_model::{
    channel::{message::MessageFlags, Message},
    http::interaction::InteractionResponseType,
    id::{
        marker::{GuildMarker, UserMarker},
        Id
    },
    user::User
};

use crate::{Framework, InteractionModal, LuroInteraction};

impl LuroInteraction for InteractionModal {
    /// Create a default embed which has the guild's accent colour if available, otherwise falls back to Luro's accent colour
    async fn default_embed<D: LuroDatabaseDriver>(&self, ctx: &Framework<D>) -> EmbedBuilder {
        ctx.default_embed(self.guild_id.as_ref()).await
    }

    /// ID of the user that invoked the interaction.
    ///
    /// This will first check for the [`member`]'s
    /// [`user`][`PartialMember::user`]'s ID and then, if not present, check the
    /// [`user`]'s ID.
    ///
    /// [`member`]: Self::member
    /// [`user`]: Self::user
    ///
    /// NOTE: Author ID is NOT present on a ping interaction. This function WILL panic if called in a ping context!
    /// Author is present on all other types
    fn author_id(&self) -> Id<UserMarker> {
        self.author().id
    }

    /// The user that invoked the interaction.
    ///
    /// This will first check for the [`member`]'s
    /// [`user`][`PartialMember::user`] and then, if not present, check the
    /// [`user`].
    ///
    /// [`member`]: Self::member
    /// [`user`]: Self::user
    ///
    /// NOTE: Author is NOT present on a ping interaction. This function WILL panic if called in a ping context!
    /// Author is present on all other types
    fn author(&self) -> &User {
        match self.member.as_ref() {
            Some(member) if member.user.is_some() => member.user.as_ref().unwrap(),
            _ => self.user.as_ref().unwrap()
        }
    }

    /// Attempts to get the guild's accent colour, else falls back to getting the hardcoded accent colour
    async fn accent_colour<D: LuroDatabaseDriver>(&self, ctx: &Framework<D>) -> u32 {
        match self.guild_id {
            Some(guild_id) => ctx
                .guild_accent_colour(&guild_id)
                .await
                .map(|x| x.unwrap_or(ACCENT_COLOUR)) // Guild has no accent colour
                .unwrap_or(ACCENT_COLOUR), // We had an error getting the guild's accent colour
            None => ACCENT_COLOUR // There is no guild for this interaction
        }
    }

    /// Create a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    /// This method returns an optional message, if the message was updated
    async fn respond_message<D, F>(&self, ctx: &Framework<D>, response: F) -> anyhow::Result<Option<Message>>
    where
        D: LuroDatabaseDriver,
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse
    {
        let mut r = LuroResponse::default();
        response(&mut r);

        match r.interaction_response_type == InteractionResponseType::DeferredChannelMessageWithSource
            || r.interaction_response_type == InteractionResponseType::DeferredUpdateMessage
        {
            true => Ok(Some(self.response_update(ctx, &r).await?)),
            false => self.response_create(ctx, &r).await
        }
    }

    /// Create a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    async fn respond<D, F>(&self, ctx: &Framework<D>, response: F) -> anyhow::Result<()>
    where
        D: LuroDatabaseDriver,
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse
    {
        let mut r = LuroResponse::default();
        response(&mut r);

        match r.interaction_response_type == InteractionResponseType::DeferredChannelMessageWithSource
            || r.interaction_response_type == InteractionResponseType::DeferredUpdateMessage
        {
            true => {
                self.response_update(ctx, &r).await?;
            }
            false => {
                self.response_create(ctx, &r).await?;
            }
        }

        Ok(())
    }

    /// Create a response. This is used for sending a response to an interaction, as well as to defer interactions.
    /// This CANNOT be used to update a response! Use `response_update` for that!
    async fn response_create<D: LuroDatabaseDriver>(
        &self,
        ctx: &Framework<D>,
        response: &LuroResponse
    ) -> anyhow::Result<Option<Message>> {
        let client = ctx.interaction_client(self.application_id);
        let request = response.interaction_response();

        match client.create_response(self.id, &self.token, &request).await {
            Ok(_) => Ok(None),
            Err(why) => {
                warn!(why = ?why, "Failed to send a response to an interaction, attempting to send as an update");
                Ok(Some(self.response_update(ctx, response).await?))
            }
        }
    }

    /// Update an existing response
    async fn response_update<D: LuroDatabaseDriver>(
        &self,
        framework: &Framework<D>,
        response: &LuroResponse
    ) -> anyhow::Result<Message> {
        Ok(framework
            .interaction_client(self.application_id)
            .update_response(&self.token)
            .allowed_mentions(response.allowed_mentions.as_ref())
            .components(response.components.as_deref())
            .content(response.content.as_deref())
            .embeds(response.embeds.as_deref())
            .await?
            .model()
            .await?)
    }

    /// Get and return useful information about the interaction author
    async fn get_interaction_author<D: LuroDatabaseDriver>(&self, ctx: &Framework<D>) -> anyhow::Result<LuroUser> {
        ctx.database.get_user(&self.author_id()).await
    }

    /// Get a specified user, else fall back to the interaction author
    /// Returns the user, their avatar and a nicely formatted name
    async fn get_specified_user_or_author<D: LuroDatabaseDriver>(
        &self,
        ctx: &Framework<D>,
        specified_user: Option<&ResolvedUser>
    ) -> anyhow::Result<LuroUser> {
        match specified_user {
            Some(user_defined) => ctx.database.get_user(&user_defined.resolved.id).await,
            None => self.get_interaction_author(ctx).await
        }
    }

    /// Acknowledge the interaction, showing a loading state. This will then be updated later.
    ///
    /// Use this for operations that take a long time. Generally its best to send this as soon as the reaction has been received.
    async fn acknowledge_interaction<D: LuroDatabaseDriver>(
        &self,
        ctx: &Framework<D>,
        ephemeral: bool
    ) -> anyhow::Result<LuroResponse> {
        let response = LuroResponse {
            interaction_response_type: InteractionResponseType::DeferredChannelMessageWithSource,
            flags: if ephemeral { Some(MessageFlags::EPHEMERAL) } else { None },
            ..Default::default()
        };

        self.response_create(ctx, &response).await?;
        Ok(response)
    }

    fn guild_id(&self) -> Option<Id<GuildMarker>> {
        self.guild_id
    }

    fn command_name(&self) -> &str {
        &self.data.custom_id
    }

    /// Send an existing response builder a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    async fn send_response<D: LuroDatabaseDriver>(
        &self,
        ctx: &Framework<D>,
        response: LuroResponse
    ) -> anyhow::Result<Option<Message>> {
        self.respond_message(ctx, |r| {
            *r = response;
            r
        })
        .await
    }
}
