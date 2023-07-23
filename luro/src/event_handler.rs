use std::{str::FromStr, sync::Arc};

use anyhow::{bail, Error};
use tracing::{debug, error, info, warn};
use twilight_gateway::{Event, MessageSender};
use twilight_model::{
    application::{
        command::Command,
        interaction::{Interaction, InteractionData, InteractionType}
    },
    id::{marker::ApplicationMarker, Id}
};

use crate::{commands::LuroCommand, models::LuroResponse};
use crate::{
    commands::{boop::BoopCommand, heck::add::HeckAddCommand, Commands},
    functions::CustomId,
    interactions::{InteractionResponder, InteractionResponse},
    models::LuroFramework,
    responses::{internal_error::internal_error_response, unknown_command::unknown_command_response},
    LuroContext, SlashResponse
};

mod ban_add;
mod message_create;
mod message_delete;
mod message_update;
mod ready;

impl LuroFramework {
    pub async fn handle_event(ctx: LuroContext, event: Event, shard: MessageSender) -> anyhow::Result<()> {
        ctx.lavalink.process(&event).await?;
        ctx.twilight_cache.update(&event);

        let callback = match event {
            Event::Ready(ready) => ctx.ready_listener(ready, shard).await,
            Event::InteractionCreate(interaction) => ctx.handle_interaction(interaction.0, shard).await,
            Event::MessageCreate(message) => ctx.message_create_listener(message).await,
            Event::MessageDelete(message) => ctx.message_delete_listener(message).await,
            Event::MessageUpdate(message) => LuroFramework::message_update_handler(message).await,
            Event::BanAdd(ban) => ctx.ban_add_listener(ban).await,
            _ => Ok(())
        };

        // TODO: Really shitty event handler, please change this
        if let Err(why) = callback {
            error!(why = ?why, "error while handling event");
        }

        Ok(())
    }

    /// Handle incoming [`Interaction`].
    pub async fn handle_interaction(self: Arc<Self>, interaction: Interaction, shard: MessageSender) -> Result<(), Error> {
        let responder = InteractionResponder::from_interaction(&interaction);
        debug!(id = ?interaction.id, "received {} interaction", interaction.kind.kind());

        let response = match interaction.kind {
            InteractionType::ApplicationCommand => self.clone().handle_command(interaction, shard).await,
            InteractionType::MessageComponent => self.handle_component(interaction).await,
            InteractionType::ModalSubmit => Self::handle_modal(interaction).await,
            other => {
                warn!("received unexpected {} interaction", other.kind());

                return Ok(());
            }
        };

        match response {
            Ok(response) => Ok(responder.respond(&self, &response).await?),
            Err(why) => {
                error!(error = ?why, "error while processing interaction");

                match responder
                    .respond(
                        &self,
                        &internal_error_response(
                            &why.to_string(),
                            LuroResponse {
                                ephemeral: true,
                                deferred: false
                            }
                        )
                    )
                    .await
                {
                    Ok(_) => info!("Successfully responded to interaction with error"),
                    Err(_) => match responder
                        .respond(
                            &self,
                            &internal_error_response(
                                &why.to_string(),
                                LuroResponse {
                                    ephemeral: true,
                                    deferred: true
                                }
                            )
                        )
                        .await
                    {
                        Ok(_) => info!("Successfully responded to interaction with error"),
                        Err(_) => error!("Failed to respond to interaction with error")
                    }
                }

                Ok(())
            }
        }
    }

    /// Handle incoming component interaction
    async fn handle_component(&self, interaction: Interaction) -> Result<InteractionResponse, anyhow::Error> {
        let custom_id = match &interaction.data {
            Some(InteractionData::MessageComponent(data)) => CustomId::from_str(&data.custom_id)?,
            _ => bail!("expected message component data")
        };

        match &*custom_id.name {
            "boop" => BoopCommand::handle_button(Default::default(), interaction).await,
            "heck-setting" => HeckAddCommand::handle_button(Default::default(), interaction).await,

            name => {
                warn!(name = name, "received unknown component");

                Ok(unknown_command_response(LuroResponse {
                    ephemeral: true,
                    deferred: false
                }))
            }
        }
    }

    /// Register commands to the Discord API.
    pub async fn register_commands(&self, application_id: Id<ApplicationMarker>) -> anyhow::Result<()> {
        let client = self.twilight_client.interaction(application_id);

        if let Err(error) = client.set_guild_commands(Id::new(234815470954348545), &[]).await {
            error!(error = ?error, "failed to clear guild commands");
        };

        match client
            .set_global_commands(
                &Commands::default_commands()
                    .global_commands
                    .into_values()
                    .collect::<Vec<Command>>()
            )
            .await
        {
            Ok(command_result) => Ok(info!(
                "Successfully registered {} global commands!",
                command_result.model().await?.len()
            )),
            Err(why) => Err(why.into())
        }
    }

    pub async fn defer_interaction(
        self: &Arc<Self>,
        interaction: &Interaction,
        ephemeral: bool
    ) -> anyhow::Result<LuroResponse> {
        debug!("Deferring interaction");
        InteractionResponder::from_interaction(interaction)
            .respond(self, &InteractionResponse::Defer { ephemeral })
            .await?;

        Ok(LuroResponse {
            ephemeral,
            deferred: true
        })
    }

    /// Handle incoming modal interaction
    async fn handle_modal(interaction: Interaction) -> SlashResponse {
        let custom_id = match &interaction.data {
            Some(InteractionData::ModalSubmit(data)) => CustomId::from_str(&data.custom_id)?,
            _ => bail!("expected modal submit data")
        };

        match &*custom_id.name {
            "heck-add" => HeckAddCommand::handle_model(Default::default(), interaction).await,
            name => {
                warn!(name = name, "received unknown modal");
                Ok(unknown_command_response(LuroResponse {
                    ephemeral: true,
                    deferred: true
                }))
            }
        }
    }
}
