use poise::serenity_prelude::{ButtonStyle, CollectComponentInteraction, InteractionResponseType};

use crate::{Context, Error};

/// Boop the bot!
#[poise::command(prefix_command, track_edits, slash_command, category = "Silly")]
pub async fn boop(ctx: Context<'_>) -> Result<(), Error> {
    let uuid_boop = ctx.id();

    ctx.send(|m| {
        m.content("I want some boops!").components(|c| {
            c.create_action_row(|ar| ar.create_button(|b| b.style(ButtonStyle::Primary).label("Boop me!").custom_id(uuid_boop)))
        })
    })
    .await?;

    let mut boop_count = 0;
    while let Some(mci) = CollectComponentInteraction::new(ctx)
        .channel_id(ctx.channel_id())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == uuid_boop.to_string())
        .await
    {
        boop_count += 1;

        let mut msg = mci.message.clone();
        msg.edit(ctx, |m| m.content(format!("Boop count: {boop_count}"))).await?;

        mci.create_interaction_response(ctx, |ir| ir.kind(InteractionResponseType::DeferredUpdateMessage))
            .await?;
    }

    Ok(())
}
