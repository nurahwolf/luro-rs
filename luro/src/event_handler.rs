use std::{str::FromStr, sync::Arc};

use anyhow::{bail, Error};
use tracing::{debug, error, info, warn};
use twilight_gateway::{Event, MessageSender};
use twilight_interactions::command::CommandModel;
use twilight_model::{
    application::{
        command::Command,
        interaction::{Interaction, InteractionData, InteractionType},
    },
    id::{marker::ApplicationMarker, Id},
};

use crate::{
    commands::{
        about::AboutCommand,
        boop::BoopCommand,
        count::CountCommand,
        heck::{
            add::{handle_heck_model, HeckAddCommand},
            HeckCommands,
        },
        hello::HelloCommand,
        moderator::ModeratorCommands,
        music::MusicCommands,
        owner::OwnerCommands,
        say::SayCommand,
    },
    framework::LuroFramework,
    functions::CustomId,
    interactions::{InteractionResponder, InteractionResponse},
    responses::embeds::{internal_error::internal_error, unknown_command::unknown_command},
    LuroContext, SlashResponse,
};

mod message_create;
mod message_delete;
mod message_update;
mod ready;
mod ban_add;

impl LuroFramework {
    pub async fn handle_event(
        ctx: LuroContext,
        event: Event,
        shard: MessageSender,
    ) -> anyhow::Result<()> {
        ctx.lavalink.process(&event).await?;
        ctx.twilight_cache.update(&event);

        match event {
            Event::Ready(ready) => ctx.ready_listener(ready, shard).await?,
            Event::InteractionCreate(interaction) => ctx.handle_interaction(interaction.0, shard).await?,
            Event::MessageCreate(message) => ctx.message_create_listener(message).await?,
            Event::MessageDelete(message) => ctx.message_delete_listener(message).await?,
            Event::MessageUpdate(message) => LuroFramework::message_update_handler(message).await?,
            Event::BanAdd(ban) => ctx.ban_add_listener(ban).await?,

            _ => (),
        };

        Ok(())
    }

    /// Handle incoming [`Interaction`].
    pub async fn handle_interaction(
        self: Arc<Self>,
        interaction: Interaction,
        shard: MessageSender,
    ) -> Result<(), Error> {
        let responder = InteractionResponder::from_interaction(&interaction);
        debug!(id = ?interaction.id, "received {} interaction", interaction.kind.kind());

        let response = match interaction.kind {
            InteractionType::ApplicationCommand => {
                self.clone().handle_command(&interaction, shard).await
            }
            InteractionType::MessageComponent => {
                self.handle_component(self.clone(), &interaction).await
            }
            InteractionType::ModalSubmit => handle_modal(interaction, &self).await,
            other => {
                warn!("received unexpected {} interaction", other.kind());

                return Ok(());
            }
        };

        match response {
            Ok(response) => Ok(responder.respond(&self, response).await?),
            Err(error) => {
                error!(error = ?error, "error while processing interaction");

                responder
                    .respond(
                        &self,
                        internal_error(format!("```{}```", error.to_string())),
                    )
                    .await
            }
        }
    }

    /// Handle incoming command interaction.
    async fn handle_command(
        self: Arc<Self>,
        interaction: &Interaction,
        shard: MessageSender,
    ) -> Result<InteractionResponse, anyhow::Error> {
        let data = match interaction.data.clone() {
            Some(InteractionData::ApplicationCommand(data)) => *data,
            _ => bail!("expected application command data"),
        };

        Ok(match data.name.as_str() {
            "about" => {
                AboutCommand::run(
                    AboutCommand::from_interaction(data.into())?,
                    &self,
                    interaction,
                )
                .await?
            }
            "say" => SayCommand::run(SayCommand::from_interaction(data.into())?).await?,
            "hello" => {
                HelloCommand::run(
                    &HelloCommand::from_interaction(data.into())?,
                    &self,
                    interaction,
                )
                .await?
            }
            "count" => {
                CountCommand::run(CountCommand::from_interaction(data.into())?, &self).await?
            }
            "mod" => ModeratorCommands::run(interaction, &self, data).await?,
            "music" => MusicCommands::run(interaction, &self, data, shard).await?,
            "boop" => BoopCommand::run().await?,
            "owner" => OwnerCommands::run(interaction, &self, data).await?,
            "heck" => {
                HeckCommands::run(
                    HeckCommands::from_interaction(data.clone().into())?,
                    self,
                    interaction,
                    data,
                )
                .await?
            }
            name => {
                warn!(name = name, "received unknown command");

                unknown_command()
            }
        })
    }

    /// Handle incoming component interaction
    async fn handle_component(
        &self,
        ctx: LuroContext,
        interaction: &Interaction,
    ) -> Result<InteractionResponse, anyhow::Error> {
        let custom_id = match &interaction.data {
            Some(InteractionData::MessageComponent(data)) => CustomId::from_str(&data.custom_id)?,
            _ => bail!("expected message component data"),
        };

        match &*custom_id.name {
            "boop" => BoopCommand::button(interaction).await,
            "heck-setting" => HeckAddCommand::handle(ctx, interaction).await,

            name => {
                warn!(name = name, "received unknown component");

                Ok(unknown_command())
            }
        }
    }

    /// Register commands to the Discord API.
    pub async fn register_commands(
        &self,
        application_id: Id<ApplicationMarker>,
    ) -> anyhow::Result<()> {
        let client = self.twilight_client.interaction(application_id);

        if let Err(error) = client
            .set_guild_commands(Id::new(234815470954348545), &[])
            .await
        {
            error!(error = ?error, "failed to clear guild commands");
        };

        match client
            .set_global_commands(
                &self
                    .commands
                    .global_commands
                    .clone()
                    .into_values()
                    .collect::<Vec<Command>>(),
            )
            .await
        {
            Ok(command_result) => Ok(info!(
                "Successfully registered {} global commands!",
                command_result.model().await?.len()
            )),
            Err(why) => Err(why.into()),
        }
    }
}

/// Handle incoming modal interaction
async fn handle_modal(interaction: Interaction, _: &LuroContext) -> SlashResponse {
    let custom_id = match &interaction.data {
        Some(InteractionData::ModalSubmit(data)) => CustomId::from_str(&data.custom_id)?,
        _ => bail!("expected modal submit data"),
    };

    match &*custom_id.name {
        "heck-add" => handle_heck_model(interaction).await,
        name => {
            warn!(name = name, "received unknown modal");
            Ok(unknown_command())
        }
    }
}
