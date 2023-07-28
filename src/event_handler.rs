use std::sync::Arc;

use tracing::{debug, error, info};
use twilight_gateway::{Event, MessageSender};
use twilight_model::{
    application::{command::Command, interaction::Interaction},
    id::{marker::ApplicationMarker, Id}
};

use crate::{
    commands::Commands,
    interactions::{InteractionResponder, InteractionResponse},
    models::LuroFramework,
    LuroContext
};
use crate::{models::LuroResponse, responses::LuroSlash};

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
            Event::InteractionCreate(interaction) => LuroSlash::new(ctx, interaction.0, shard).handle().await,
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
    pub async fn handle_interaction() {
        // TODO: Better error handling
        // match response {
        //     Ok(response) => Ok(()),
        //     Err(why) => {

        //         match responder
        //             .respond(
        //                 &self,
        //                 &internal_error_response(
        //                     &why.to_string(),
        //                     LuroResponse {
        //                         ephemeral: true,
        //                         deferred: false
        //                     }
        //                 )
        //             )
        //             .await
        //         {
        //             Ok(_) => info!("Successfully responded to interaction with error"),
        //             Err(_) => match responder
        //                 .respond(
        //                     &self,
        //                     &internal_error_response(
        //                         &why.to_string(),
        //                         LuroResponse {
        //                             ephemeral: true,
        //                             deferred: true
        //                         }
        //                     )
        //                 )
        //                 .await
        //             {
        //                 Ok(_) => info!("Successfully responded to interaction with error"),
        //                 Err(_) => error!("Failed to respond to interaction with error")
        //             }
        //         }

        //         Ok(())
        //     }
        // }
    }

    /// Register commands to the Discord API.
    pub async fn register_commands(&self, application_id: Id<ApplicationMarker>) -> anyhow::Result<()> {
        let client = self.twilight_client.interaction(application_id);

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
}
