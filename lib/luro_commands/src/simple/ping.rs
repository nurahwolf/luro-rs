use chrono::Utc;
use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::ShardId;

use luro_core::{Context, Error};

/// Shows current latency of the bot
#[poise::command(prefix_command, slash_command, category = "General")]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let accent_colour = ctx.data().config.read().await.accent_colour;
    let start = Utc::now();
    let start_ts = start.timestamp();
    let start_ts_ss = start.timestamp_subsec_millis() as i64;
    let ping = ctx.send(|m| m.content(":ping_pong: Pinging!")).await?;
    let end = Utc::now();
    let end_ts = end.timestamp();
    let end_ts_ss = end.timestamp_subsec_millis() as i64;
    let api_response = ((end_ts - start_ts) * 1000) + (end_ts_ss - start_ts_ss);
    let shard_manager = ctx.framework().shard_manager();

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;
    let runner = runners
        .get(&ShardId(ctx.serenity_context().shard_id))
        .ok_or("No shard found")?;

    let shard_response = match runner.latency {
        Some(latency) => format!("`{}ms`", latency.as_millis()),
        None => "No data available at the moment.".to_string()
    };

    let response = format!(
        "Pong! Succesfully retrieved the message and shard latencies. :ping_pong:\n\n\
        **API Response Time**: `{api_response}ms`\n\
        **Shard Response Time**: {shard_response}"
    );

    ping.edit(ctx, |message| {
        message.content("");
        message.embed(|embed| {
            embed.colour(guild_accent_colour(accent_colour, ctx.guild()));
            embed.title("Discord Latency Information");
            embed.description(response)
        })
    })
    .await?;

    Ok(())
}
