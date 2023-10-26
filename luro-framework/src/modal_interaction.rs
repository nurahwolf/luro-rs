use std::sync::Arc;

use anyhow::{anyhow, Context};

mod command_name;
mod interaction_client;
mod parse_field;
mod respond;
mod respond_create;
mod respond_update;
mod response_simple;

use luro_database::LuroDatabase;
use luro_model::{response::LuroResponse, ACCENT_COLOUR};

use twilight_model::{
    application::interaction::{modal::ModalInteractionData, Interaction, InteractionData},
    http::interaction::InteractionResponseType,
    id::{marker::GuildMarker, Id},
};

use crate::{InteractionTrait, Luro, LuroContext};

/// A context spawned from a modal interaction
#[derive(Debug, Clone)]
pub struct ModalInteraction {
    pub app_permissions: Option<twilight_model::guild::Permissions>,
    pub application_id: Id<twilight_model::id::marker::ApplicationMarker>,
    /// The author of this interaction. Contains member data if this interaction was spawned in a guild.
    pub author: luro_database::LuroUser,
    pub cache: Arc<twilight_cache_inmemory::InMemoryCache>,
    pub channel: twilight_model::channel::Channel,
    pub data: ModalInteractionData,
    pub database: Arc<luro_database::LuroDatabase>,
    /// Information on the guild this interaction was spaned in.
    pub guild: Option<luro_database::LuroGuild>,
    pub http_client: Arc<hyper::Client<hyper::client::HttpConnector>>,
    pub id: Id<twilight_model::id::marker::InteractionMarker>,
    pub interaction_token: String,
    pub kind: twilight_model::application::interaction::InteractionType,
    pub latency: twilight_gateway::Latency,
    #[cfg(feature = "lavalink")]
    pub lavalink: Arc<twilight_lavalink::Lavalink>,
    /// The locale of this interaction
    pub locale: String,
    pub shard: twilight_gateway::MessageSender,
    pub tracing_subscriber: tracing_subscriber::reload::Handle<tracing_subscriber::filter::LevelFilter, tracing_subscriber::Registry>,
    pub twilight_client: Arc<twilight_http::Client>,
    pub user: Option<twilight_model::user::User>,
}

impl Luro for ModalInteraction {
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
    async fn respond<F>(&self, response: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut LuroResponse) -> &mut LuroResponse + Send,
    {
        let mut r = LuroResponse::default();
        response(&mut r);

        match r.interaction_response_type == InteractionResponseType::DeferredChannelMessageWithSource
            || r.interaction_response_type == InteractionResponseType::DeferredUpdateMessage
        {
            true => {
                self.response_update(&r).await?;
            }
            false => {
                self.response_create(&r).await?;
            }
        }

        Ok(())
    }

    async fn interaction_client(&self) -> anyhow::Result<twilight_http::client::InteractionClient> {
        Ok(self.twilight_client.interaction(self.application_id))
    }

    fn database(&self) -> std::sync::Arc<LuroDatabase> {
        self.database.clone()
    }

    fn twilight_client(&self) -> std::sync::Arc<twilight_http::Client> {
        self.twilight_client.clone()
    }

    fn cache(&self) -> std::sync::Arc<twilight_cache_inmemory::InMemoryCache> {
        self.cache.clone()
    }
}

impl InteractionTrait for ModalInteraction {
    fn command_name(&self) -> &str {
        &self.data.custom_id
    }
}

impl ModalInteraction {
    pub async fn new(ctx: LuroContext, interaction: Interaction) -> anyhow::Result<Self> {
        let data = match interaction
            .data
            .clone()
            .context("Attempting to create an 'ModalInteraction' from an interaction that does not have any command data")?
        {
            InteractionData::ModalSubmit(data) => data,
            _ => {
                return Err(anyhow!(
                    "Incorrect command data, meant to get ModalSubmit but actually got {:#?}",
                    interaction
                ))
            }
        };
        Ok(ModalInteraction {
            author: match interaction.guild_id {
                Some(guild_id) => {
                    ctx.database
                        .get_member(interaction.author_id().context("Expected to get author")?, guild_id)
                        .await?
                }
                None => {
                    ctx.database
                        .get_user(interaction.author_id().context("Expected to get author")?)
                        .await?
                }
            },
            app_permissions: interaction.app_permissions,
            application_id: interaction.application_id,
            cache: ctx.cache.clone(),
            channel: interaction.channel.clone().unwrap(),
            data,
            database: ctx.database.clone(),
            guild: match interaction.guild_id {
                Some(guild_id) => Some(ctx.database.get_guild(guild_id).await?),
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
            interaction_token: interaction.token.clone(),
            tracing_subscriber: ctx.tracing_subscriber,
            twilight_client: ctx.twilight_client,
            user: interaction.user.clone(),
        })
    }
}
