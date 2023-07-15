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
        boop::BoopCommand, count::CountCommand, heck::HeckCommands, hello::HelloCommand,
        moderator::ModeratorCommands, music::MusicCommands, say::SayCommand,
    },
    framework::LuroFramework,
    functions::CustomId,
    interactions::{InteractionResponder, InteractionResponse},
    responses::embeds::{internal_error::internal_error, unknown_command::unknown_command},
    LuroContext,
};

mod message_create;
mod message_delete;
mod message_update;
mod ready;

impl LuroFramework {
    pub async fn handle_event(
        ctx: LuroContext,
        event: Event,
        shard: MessageSender,
    ) -> anyhow::Result<()> {
        ctx.lavalink.process(&event).await?;

        match event {
            Event::Ready(ready) => ctx.ready_listener(ready).await?,
            Event::InteractionCreate(interaction) => {
                ctx.handle_interaction(interaction.0, shard).await?
            }
            Event::MessageCreate(message) => {
                LuroFramework::message_create_listener(message).await?
            }
            Event::MessageDelete(message) => ctx.message_delete_listener(message).await?,
            Event::MessageUpdate(message) => LuroFramework::message_update_handler(message).await?,
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
            InteractionType::MessageComponent => self.handle_component(&interaction).await,
            // InteractionType::ModalSubmit => handle_modal(interaction, ctx).await,
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

        match data.name.as_str() {
            "say" => Ok(SayCommand::run(SayCommand::from_interaction(data.into())?).await?),
            "hello" => Ok(HelloCommand::execute(
                &HelloCommand::from_interaction(data.into())?,
                &self,
                interaction,
            )
            .await?),
            "count" => {
                Ok(CountCommand::run(CountCommand::from_interaction(data.into())?, &self).await?)
            }
            "mod" => Ok(ModeratorCommands::run(interaction, &self, data).await?),
            "music" => Ok(MusicCommands::run(interaction, &self, data, shard).await?),
            "boop" => Ok(BoopCommand::run().await?),
            "heck" => Ok(HeckCommands::run(
                HeckCommands::from_interaction(data.clone().into())?,
                self,
                interaction,
                data,
            )
            .await?),
            name => {
                warn!(name = name, "received unknown command");

                Ok(unknown_command())
            }
        }
    }

    /// Handle incoming component interaction
    async fn handle_component(
        &self,
        interaction: &Interaction,
    ) -> Result<InteractionResponse, anyhow::Error> {
        let custom_id = match &interaction.data {
            Some(InteractionData::MessageComponent(data)) => CustomId::from_str(&data.custom_id)?,
            _ => bail!("expected message component data"),
        };

        match &*custom_id.name {
            "boop" => BoopCommand::button(interaction).await,
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

// /// Handle incoming modal interaction
// async fn handle_modal(
//     interaction: Interaction,
//     ctx: &LuroFramework,
// ) -> Result<InteractionResponse, anyhow::Error> {
//     let custom_id = match &interaction.data {
//         Some(InteractionData::ModalSubmit(data)) => CustomId::from_str(&*data.custom_id)?,
//         _ => bail!("expected modal submit data"),
//     };

//     match &*custom_id.name {
//         "captcha-modal" => CaptchaModal::handle(interaction, ctx).await,
//         // "sanction" => bail!("not implemented"),
//         name => {
//             warn!(name = name, "received unknown modal");

//             Ok(embed::error::unknown_command(interaction.lang()?))
//         }
//     }
// }
