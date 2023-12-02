use std::sync::Arc;

use anyhow::{anyhow, Context};
use luro_model::{response::InteractionResponse, types::CommandResponse, ACCENT_COLOUR};
use twilight_model::{
    application::interaction::{message_component::MessageComponentInteractionData, Interaction, InteractionData},
    channel::Message,
    http::interaction::InteractionResponseType,
    id::{marker::GuildMarker, Id},
};

use crate::{InteractionTrait, Luro, LuroContext};

mod command_name;
mod get_specific_user_or_author;
mod interaction_client;
mod respond;
mod respond_create;
mod respond_message;
mod response_send;
mod response_update;
mod send_message;
mod simple_response;

#[derive(Debug, Clone)]
pub struct ComponentInteraction {
    pub app_permissions: Option<twilight_model::guild::Permissions>,
    pub application_id: Id<twilight_model::id::marker::ApplicationMarker>,
    /// The author of this interaction. Contains member data if this interaction was spawned in a guild.
    pub author: luro_model::types::User,
    pub channel: twilight_model::channel::Channel,
    pub data: Box<MessageComponentInteractionData>,
    pub database: Arc<luro_database::Database>,
    /// Information on the guild this interaction was spaned in.
    pub guild: Option<luro_model::types::Guild>,
    pub http_client: Arc<hyper::Client<hyper::client::HttpConnector>>,
    pub id: Id<twilight_model::id::marker::InteractionMarker>,
    pub interaction_token: String,
    pub kind: twilight_model::application::interaction::InteractionType,
    pub latency: twilight_gateway::Latency,
    #[cfg(feature = "lavalink")]
    pub lavalink: Arc<twilight_lavalink::Lavalink>,
    /// The locale of this interaction
    pub locale: String,
    pub message: Message,
    pub shard: twilight_gateway::MessageSender,
    pub logging: Arc<luro_logging::Logging>,
    pub twilight_client: Arc<twilight_http::Client>,
    pub user: Option<twilight_model::user::User>,
}

impl Luro for ComponentInteraction {
    fn accent_colour(&self) -> u32 {
        match &self.guild {
            Some(guild) => match &guild.data {
                Some(guild) => guild.accent_colour_custom.unwrap_or(guild.accent_colour.unwrap_or(ACCENT_COLOUR)),
                None => ACCENT_COLOUR,
            },
            None => ACCENT_COLOUR, // There is no guild for this interaction
        }
    }

    fn guild_id(&self) -> Option<Id<GuildMarker>> {
        self.guild.as_ref().map(|x| x.guild_id)
    }

    /// Create a response to an interaction.
    /// This automatically handles if the interaction had been deferred.
    async fn respond<F>(&self, response: F) -> anyhow::Result<CommandResponse>
    where
        F: FnOnce(&mut InteractionResponse) -> &mut InteractionResponse + Send,
    {
        let mut r = InteractionResponse::default();
        response(&mut r);

        match r.interaction_response_type == InteractionResponseType::DeferredChannelMessageWithSource
            || r.interaction_response_type == InteractionResponseType::DeferredUpdateMessage
        {
            true => self.response_update(&r).await,
            false => self.response_create(&r).await,
        }
    }

    async fn interaction_client(&self) -> anyhow::Result<twilight_http::client::InteractionClient> {
        Ok(self.twilight_client.interaction(self.application_id))
    }

    fn database(&self) -> std::sync::Arc<luro_database::Database> {
        self.database.clone()
    }

    fn twilight_client(&self) -> std::sync::Arc<twilight_http::Client> {
        self.twilight_client.clone()
    }
}

impl InteractionTrait for ComponentInteraction {
    fn command_name(&self) -> &str {
        &self.data.custom_id
    }
}

impl ComponentInteraction {
    pub async fn new(ctx: LuroContext, interaction: Interaction) -> anyhow::Result<Self> {
        let data = match interaction
            .data
            .clone()
            .context("Attempting to create an 'ComponentInteraction' from an interaction that does not have any command data")?
        {
            InteractionData::MessageComponent(data) => data,
            _ => {
                return Err(anyhow!(
                    "Incorrect command data, meant to get MessageComponent but actually got {:#?}",
                    interaction
                ))
            }
        };
        Ok(ComponentInteraction {
            author: match interaction.guild_id {
                Some(guild_id) => {
                    ctx.database
                        .member_fetch(guild_id, interaction.author_id().context("Expected to get author")?)
                        .await?
                }
                None => {
                    ctx.database
                        .user_fetch(interaction.author_id().context("Expected to get author")?)
                        .await?
                }
            },
            app_permissions: interaction.app_permissions,
            application_id: interaction.application_id,
            channel: interaction.channel.clone().unwrap(),
            data,
            database: ctx.database.clone(),
            guild: match interaction.guild_id {
                Some(guild_id) => Some(ctx.database.guild_fetch(guild_id).await?),
                None => None,
            },
            http_client: ctx.http_client,
            id: interaction.id,
            kind: interaction.kind,
            latency: ctx.latency,
            #[cfg(feature = "lavalink")]
            lavalink: ctx.lavalink,
            locale: interaction.locale.clone().context("Expected to get interaction locale")?,
            shard: ctx.shard,
            message: interaction.message.context("Expected to fetch message on component interaction")?,
            interaction_token: interaction.token.clone(),
            logging: ctx.logging,
            twilight_client: ctx.twilight_client,
            user: interaction.user.clone(),
        })
    }
}
