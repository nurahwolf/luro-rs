use std::sync::Arc;

use anyhow::{anyhow, Context};

mod author;
mod command_name;
mod interaction_client;
mod parse_field;
mod respond;
mod respond_create;
mod respond_update;
mod response_simple;

use luro_database::LuroDatabase;
use luro_model::{response::LuroResponse, ACCENT_COLOUR};
use tracing::warn;
use twilight_model::{
    application::interaction::{modal::ModalInteractionData, Interaction, InteractionData},
    http::interaction::InteractionResponseType,
    id::{marker::GuildMarker, Id},
    user::User,
};

use crate::{Luro, InteractionTrait, LuroContext};

/// A context spawned from a modal interaction
#[derive(Debug, Clone)]
pub struct ModalInteraction {
    pub app_permissions: Option<twilight_model::guild::Permissions>,
    pub application_id: Id<twilight_model::id::marker::ApplicationMarker>,
    pub cache: Arc<twilight_cache_inmemory::InMemoryCache>,
    pub channel: twilight_model::channel::Channel,
    pub data: ModalInteractionData,
    pub database: Arc<LuroDatabase>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub guild_locale: Option<String>,
    pub http_client: Arc<hyper::Client<hyper::client::HttpConnector>>,
    pub id: Id<twilight_model::id::marker::InteractionMarker>,
    pub kind: twilight_model::application::interaction::InteractionType,
    pub latency: twilight_gateway::Latency,
    #[cfg(feature = "lavalink")]
    pub lavalink: Arc<twilight_lavalink::Lavalink>,
    pub locale: Option<String>,
    pub member: Option<twilight_model::guild::PartialMember>,
    pub message: Option<twilight_model::channel::Message>,
    pub original: twilight_model::application::interaction::Interaction,
    pub shard: twilight_gateway::MessageSender,
    pub token: String,
    pub tracing_subscriber: tracing_subscriber::reload::Handle<tracing_subscriber::filter::LevelFilter, tracing_subscriber::Registry>,
    pub twilight_client: Arc<twilight_http::Client>,
    pub user: Option<twilight_model::user::User>,
}

impl Luro for ModalInteraction {
    fn guild_id(&self) -> Option<Id<GuildMarker>> {
        self.guild_id
    }

    async fn accent_colour(&self) -> u32 {
        match self.guild_id {
            Some(guild_id) => match self.get_guild(&guild_id).await.map(|mut x| x.highest_role_colour().map(|x| x.0)) {
                Ok(colour) => colour.unwrap_or(ACCENT_COLOUR),
                Err(why) => {
                    warn!(why = ?why, "Failed to get guild accent colour");
                    ACCENT_COLOUR
                }
            },
            None => ACCENT_COLOUR, // There is no guild for this interaction
        }
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

    /// The user that invoked the interaction.
    ///
    /// This will first check for the [`member`]'s
    /// [`user`][`PartialMember::user`] and then, if not present, check the
    /// [`user`].
    ///
    /// [`member`]: Self::member
    /// [`user`]: Self::user
    fn author(&self) -> &User {
        match self.member.as_ref() {
            Some(member) if member.user.is_some() => member.user.as_ref().unwrap(),
            _ => self.user.as_ref().unwrap(),
        }
    }
}

impl ModalInteraction {
    pub fn new(ctx: LuroContext, interaction: Interaction) -> anyhow::Result<Self> {
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
            app_permissions: interaction.app_permissions,
            application_id: interaction.application_id,
            cache: ctx.cache,
            channel: interaction.channel.clone().unwrap(),
            data,
            database: ctx.database,
            guild_id: interaction.guild_id,
            guild_locale: interaction.guild_locale.clone(),
            http_client: ctx.http_client,
            id: interaction.id,
            kind: interaction.kind,
            latency: ctx.latency,
            #[cfg(feature = "lavalink")]
            lavalink: ctx.lavalink,
            locale: interaction.locale.clone(),
            member: interaction.member.clone(),
            message: interaction.message.clone(),
            original: interaction.clone(),
            shard: ctx.shard,
            token: interaction.token.clone(),
            tracing_subscriber: ctx.tracing_subscriber,
            twilight_client: ctx.twilight_client,
            user: interaction.user.clone(),
        })
    }
}
