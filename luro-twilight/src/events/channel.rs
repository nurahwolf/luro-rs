use luro_framework::LuroContext;
use twilight_model::gateway::payload::incoming::{ChannelCreate, ChannelDelete, ChannelPinsUpdate, ChannelUpdate};

pub async fn create(ctx: LuroContext, event: Box<ChannelCreate>) -> anyhow::Result<()> {
    #[cfg(feature = "pretty-tables")]
    let mut builder = tabled::builder::Builder::new();
    if let Some(name) = &event.name {
        builder.push_record(["Name", name]);
    }
    builder.push_record(["ID", &event.id.to_string()]);
    builder.push_record(["Kind", &event.kind.name()]);

    tracing::info!(
        "-- Channel Created -- \n{}",
        builder.build().with(tabled::settings::Style::ascii_rounded()).to_string()
    );

    #[cfg(not(feature = "pretty-tables"))]
    tracing::debug!("Channel {} created", event.id);

    ctx.database.channel_update(event).await?;

    Ok(())
}

pub async fn pins_update(ctx: LuroContext, event: ChannelPinsUpdate) -> anyhow::Result<()> {
    #[cfg(feature = "pretty-tables")]
    let mut builder = tabled::builder::Builder::new();
    builder.push_record(["ID", &event.channel_id.to_string()]);
    if let Some(guild_id) = event.guild_id {
        builder.push_record(["Guild ID", &guild_id.to_string()]);
    }

    tracing::info!(
        "-- Channel Pins Updated -- \n{}",
        builder.build().with(tabled::settings::Style::ascii_rounded()).to_string()
    );

    #[cfg(not(feature = "pretty-tables"))]
    tracing::debug!("Channel {} pins updated", event.channel_id);

    ctx.database.channel_update(event).await?;

    Ok(())
}

pub async fn delete(ctx: LuroContext, event: Box<ChannelDelete>) -> anyhow::Result<()> {
    #[cfg(feature = "pretty-tables")]
    let mut builder = tabled::builder::Builder::new();
    if let Some(name) = &event.name {
        builder.push_record(["Name", name]);
    }
    builder.push_record(["ID", &event.id.to_string()]);
    builder.push_record(["Kind", &event.kind.name()]);

    tracing::info!(
        "-- Channel Deleted -- \n{}",
        builder.build().with(tabled::settings::Style::ascii_rounded()).to_string()
    );

    #[cfg(not(feature = "pretty-tables"))]
    tracing::debug!("Channel {} deleted", event.id);

    ctx.database.channel_update(event).await?;

    Ok(())
}

pub async fn update(ctx: LuroContext, event: Box<ChannelUpdate>) -> anyhow::Result<()> {
    #[cfg(feature = "pretty-tables")]
    let mut builder = tabled::builder::Builder::new();
    if let Some(name) = &event.name {
        builder.push_record(["Name", name]);
    }
    builder.push_record(["ID", &event.id.to_string()]);
    builder.push_record(["Kind", &event.kind.name()]);

    tracing::info!(
        "-- Channel Updated -- \n{}",
        builder.build().with(tabled::settings::Style::ascii_rounded()).to_string()
    );

    #[cfg(not(feature = "pretty-tables"))]
    tracing::debug!("Channel {} updated", event.id);

    ctx.database.channel_update(event).await?;

    Ok(())
}
