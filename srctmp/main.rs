use anyhow::Context;
use futures::StreamExt;
use std::env;
use std::sync::Arc;
use tracing::info;
use twilight_gateway::stream::{self, ShardEventStream};
use twilight_gateway::ConfigBuilder;
use twilight_model::gateway::event::Event;
use twilight_model::gateway::Intents;
use twilight_model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};
use twilight_model::id::marker::{ApplicationMarker, GuildMarker};
use twilight_model::id::Id;
use zephyrus::prelude::*;

pub mod models;

#[command]
#[description = "Says hello"]
async fn hello(ctx: &SlashContext<()>) -> DefaultCommandResult {
    ctx.interaction_client
        .create_response(
            ctx.interaction.id,
            &ctx.interaction.token,
            &InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(InteractionResponseData {
                    content: Some(String::from("Hello world")),
                    ..Default::default()
                }),
            },
        )
        .await?;

    Ok(())
}

async fn handle_events(
    event: Event,
    framework: Arc<Framework<()>>,
) {
    // Zephyrus can register commands in guilds or globally.

    match event {
        Event::Ready(ready) => {
            info!("Luro is ready!");
            framework
                .register_guild_commands(Id::<GuildMarker>::new(234815470954348545))
                .await
                .unwrap();
        }
        Event::InteractionCreate(i) => {
            tokio::spawn(async move {
                let inner = i.0;
                framework.process(inner).await;
            });
        }
        _ => (),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialise the tracing subscriber.
    tracing_subscriber::fmt::init();
    tracing::info!("Booting Luro!");

    let app_id = Id::<ApplicationMarker>::new(180285835399266304);
    let (token, lavalink_host, lavalink_auth, intents) = (
        env::var("DISCORD_TOKEN").context("Failed to get the variable DISCORD_TOKEN")?,
        env::var("LAVALINK_HOST").context("Failed to get the variable LAVALINK_HOST")?,
        env::var("LAVALINK_AUTHORISATION")
            .context("Failed to get the variable LAVALINK_AUTHORISATION")?,
        Intents::GUILD_MESSAGES
            | Intents::GUILD_VOICE_STATES
            | Intents::MESSAGE_CONTENT
            | Intents::GUILD_INVITES,
    );

    let (twilight_client, config) = (
        Arc::new(twilight_http::Client::new(token.clone())),
        ConfigBuilder::new(token.clone(), intents).build(),
    );

    let framework = Arc::new(
        Framework::builder(twilight_client.clone(), app_id, ())
            .command(hello)
            .command(hellov2)
            .build(),
    );

    let mut shards = stream::create_recommended(&twilight_client, config, |_, c| c.build())
        .await?
        .collect::<Vec<_>>();

    let mut stream = ShardEventStream::new(shards.iter_mut());

    while let Some((shard, event)) = stream.next().await {
        let event = match event {
            Ok(event) => event,
            Err(source) => {
                tracing::warn!(?source, "error receiving event");

                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };

        tracing::info!(?event, shard = ?shard.id(), "received event");
        handle_events(event, framework.clone()).await;
    }

    Ok(())
}