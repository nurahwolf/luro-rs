// This is a module of commands that only activate on certain keywords. These can be extremely spammy, so should be activated only in allowlisted conditions
mod lily;
mod luro;
mod mention;
mod paw;

/// Handle commands that are invoked by a 'keyword' being present in message content.
pub async fn keyword_handler(ctx: crate::models::CommandContext) {
    // Don't respond if the user is a bot
    if ctx.data.author.bot {
        return;
    }

    // TODO: Conditionally check we are in a guild
    if let Some(guild_id) = ctx.data.guild_id {
        // Only respond in Nurah's dungeon. Also short circuit if we are in the venting channel
        if guild_id.get() != 1132063963337740379 || ctx.data.channel_id.get() == 1139232506990833725
        {
            return;
        }
    }

    // Client to send messages
    let message_client = ctx
        .twilight_client
        .create_message(ctx.data.channel_id)
        .reply(ctx.data.id);

    let response =
        if !ctx.data.mentions.is_empty() && ctx.data.author.id.get() == 307419691268440064 {
            // If Luro (307419691268440064) responds to someone, be a little shit
            message_client.content(luro::choose_insult()).await
        } else if ctx
            .data
            .mentions
            .iter()
            .any(|x| x.id == ctx.current_user.id)
        {
            // If anyone responds to Sira (Message Reply), be a little shit
            message_client.content(mention::choose_insult()).await
        } else {
            // Parse a few words for some silly responses if they are present.
            match ctx.data.content.to_lowercase() {
                x if x.contains("fops") => {
                    message_client.content("Best fops is Stripe Rose!").await
                }
                x if x.contains("owo") => {
                    message_client
                        .content("Oh no, someone is speaking furry trash in public...")
                        .await
                }
                x if x.contains("uwu") => {
                    message_client
                        .content("I'll show you some 'UwU' if you don't shush it.")
                        .await
                }
                x if x.contains("horny") => {
                    message_client
                        .content("So very lewd! So very horny uwu")
                        .await
                }
                x if x.contains("sus") => message_client
                    .content(
                        "Sus? Amongus!? I'll show you some sus by stuffing my paws in your face.",
                    )
                    .await,
                x if x.contains("paw") => message_client.content(paw::choose_insult()).await,
                x if x.contains("lily") => message_client.content(lily::choose_insult()).await,
                x if x.contains("sira") && ctx.data.author.id.get() == 307419691268440064 => {
                    message_client.content(luro::choose_insult()).await
                }
                x if x.contains("sira") => message_client.content(mention::choose_insult()).await,
                _ => return,
            }
        };

    // A remarkably crude error handler
    if let Err(why) = response {
        tracing::error!(
            ?why,
            "keyword_handler - failed to send message in response to keyword trigger"
        )
    }
}
