use anyhow::{bail, Error};
use tracing::{debug, error, info, warn};
use twilight_gateway::Event;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::{
        command::Command,
        interaction::{Interaction, InteractionData, InteractionType},
    },
    id::{marker::ApplicationMarker, Id},
};

use crate::{
    commands::{
        count::CountCommand, hello::HelloCommand, moderator::ModeratorCommands, say::SayCommand,
    },
    framework::LuroFramework,
    interactions::{InteractionResponder, InteractionResponse},
    responses::embeds::{internal_error::internal_error, unknown_command::unknown_command},
};

mod message_create;
mod message_delete;
mod message_update;
mod ready;

impl LuroFramework {
    pub async fn handle_event(&self, event: Event) -> Result<(), Error> {
        match event {
            Event::Ready(ready) => self.ready_listener(ready).await?,
            Event::InteractionCreate(interaction) => self.handle_interaction(interaction.0).await?,
            Event::MessageCreate(message) => {
                LuroFramework::message_create_listener(message).await?
            }
            Event::MessageDelete(message) => self.message_delete_listener(message).await?,
            Event::MessageUpdate(message) => LuroFramework::message_update_handler(message).await?,
            _ => (),
        };

        Ok(())
    }

    /// Handle incoming [`Interaction`].
    pub async fn handle_interaction(&self, interaction: Interaction) -> Result<(), Error> {
        let responder = InteractionResponder::from_interaction(&interaction);
        debug!(id = ?interaction.id, "received {} interaction", interaction.kind.kind());

        let response = match interaction.kind {
            InteractionType::ApplicationCommand => self.handle_command(interaction).await,
            // InteractionType::MessageComponent => handle_component(interaction, ctx).await,
            // InteractionType::ModalSubmit => handle_modal(interaction, ctx).await,
            other => {
                warn!("received unexpected {} interaction", other.kind());

                return Ok(());
            }
        };

        match response {
            Ok(response) => Ok(responder.respond(self, response).await?),
            Err(error) => {
                error!(error = ?error, "error while processing interaction");

                responder
                    .respond(self, internal_error(format!("```{}```", error.to_string())))
                    .await
            }
        }
    }

    /// Handle incoming command interaction.
    async fn handle_command(
        &self,
        interaction: Interaction,
    ) -> Result<InteractionResponse, anyhow::Error> {
        info!("Got `ApplicationCommand`!");
        let data = match interaction.data.clone() {
            Some(InteractionData::ApplicationCommand(data)) => *data,
            _ => bail!("expected application command data"),
        };

        match data.name.as_str() {
            "say" => Ok(SayCommand::run(SayCommand::from_interaction(data.into())?).await?),
            "hello" => Ok(HelloCommand::execute(
                HelloCommand::from_interaction(data.into())?,
                self,
                interaction,
            )
            .await?),
            "count" => {
                Ok(CountCommand::run(CountCommand::from_interaction(data.into())?, self).await?)
            }
            "mod" => Ok(ModeratorCommands::handle(interaction, data, self).await?),
            name => {
                warn!(name = name, "received unknown command");

                Ok(unknown_command())
            }
        }
    }

    /// Register commands to the Discord API.
    pub async fn register_commands(&self, application_id: Id<ApplicationMarker>) {
        let commands: Vec<Command> = vec![
            ModeratorCommands::create_command().into(),
            SayCommand::create_command().into(),
            HelloCommand::create_command().into(),
            CountCommand::create_command().into(),
        ];

        let client = self.twilight_client.interaction(application_id);

        if let Err(error) = client
            .set_guild_commands(Id::new(234815470954348545), &[])
            .await
        {
            error!(error = ?error, "failed to clear guild commands");
        };

        if let Err(error) = client.set_global_commands(&commands).await {
            error!(error = ?error, "failed to register commands");
        }
    }
}

// /// Handle incoming component interaction
// async fn handle_component(
//     interaction: Interaction,
//     ctx: &LuroFramework,
// ) -> Result<InteractionResponse, anyhow::Error> {
//     let custom_id = match &interaction.data {
//         Some(InteractionData::MessageComponent(data)) => CustomId::from_str(&*data.custom_id)?,
//         _ => bail!("expected message component data"),
//     };

//     match &*custom_id.name {
//         "captcha-disable" => CaptchaDisable::handle(interaction, ctx).await,
//         "captcha-enable" => CaptchaEnable::handle(interaction, ctx).await,
//         "captcha-validate" => CaptchaValidateButton::handle(interaction, ctx).await,
//         "captcha-verify" => CaptchaVerifyButton::handle(interaction, ctx).await,
//         "post-in-chat" => PostInChat::handle(interaction, custom_id, ctx).await,
//         name => {
//             warn!(name = name, "received unknown component");

//             Ok(unknown_command())
//         }
//     }
// }

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
