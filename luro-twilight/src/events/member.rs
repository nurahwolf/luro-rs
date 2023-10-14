use luro_framework::Context;
use twilight_model::gateway::payload::incoming::{MemberAdd, MemberChunk, MemberRemove, MemberUpdate};

pub async fn update(ctx: Context, event: Box<MemberUpdate>) -> anyhow::Result<()> {
    #[cfg(feature = "pretty-tables")]
    let mut builder = tabled::builder::Builder::new();
    builder.push_record(["ID", &event.user.id.to_string()]);
    builder.push_record(["Guild ID", &event.guild_id.to_string()]);

    tracing::info!(
        "-- Member Updated -- \n{}",
        builder.build().with(tabled::settings::Style::ascii_rounded()).to_string()
    );

    #[cfg(not(feature = "pretty-tables"))]
    tracing::debug!("Member {} updated", event.user.id);

    ctx.database.update_member(event).await?;

    Ok(())
}

pub async fn add(ctx: Context, event: Box<MemberAdd>) -> anyhow::Result<()> {
    #[cfg(feature = "pretty-tables")]
    let mut builder = tabled::builder::Builder::new();
    builder.push_record(["ID", &event.user.id.to_string()]);
    builder.push_record(["Guild ID", &event.guild_id.to_string()]);

    tracing::info!(
        "-- Member Added -- \n{}",
        builder.build().with(tabled::settings::Style::ascii_rounded()).to_string()
    );

    #[cfg(not(feature = "pretty-tables"))]
    tracing::debug!("Member {} added", event.user.id);

    ctx.database.update_member(event).await?;

    Ok(())
}

pub async fn delete(ctx: Context, event: MemberRemove) -> anyhow::Result<()> {
    #[cfg(feature = "pretty-tables")]
    let mut builder = tabled::builder::Builder::new();
    builder.push_record(["ID", &event.user.id.to_string()]);
    builder.push_record(["Guild ID", &event.guild_id.to_string()]);

    tracing::info!(
        "-- Member Removed -- \n{}",
        builder.build().with(tabled::settings::Style::ascii_rounded()).to_string()
    );

    #[cfg(not(feature = "pretty-tables"))]
    tracing::debug!("Member {} removed", event.user.id);

    ctx.database.update_member(event).await?;

    Ok(())
}

pub async fn chunk(ctx: Context, event: MemberChunk) -> anyhow::Result<()> {
    #[cfg(feature = "pretty-tables")]
    let mut builder = tabled::builder::Builder::new();
    builder.push_record(["IDs", &format!("{:#?}", event.members)]);
    builder.push_record(["Guild ID", &event.guild_id.to_string()]);

    tracing::info!(
        "-- Member Chunk -- \n{}",
        builder.build().with(tabled::settings::Style::ascii_rounded()).to_string()
    );

    ctx.database.update_member(event).await?;

    Ok(())
}
