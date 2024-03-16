use crate::models::message_context::MessageContext;

mod hi;
mod uwu;

/// Handle a prefix command. Returns true if a command was matched, otherwise returns false
pub async fn prefix_handler(framework: &mut MessageContext) {
    // Is a prefix defined?
    let prefix = match &framework.gateway.config.prefix {
        Some(prefix) => prefix,
        None => return,
    };

    // A prefix is defined! Does the message start with our prefix?
    let message_content = framework.ctx.content.clone();
    let message_content = message_content.split_once(' ');
    let command = match message_content {
        Some((first_word, rest_of_string)) => match &first_word.starts_with(prefix) {
            true => {
                framework.ctx.content = rest_of_string.to_owned();
                first_word.split_at(prefix.len()).1
            } // Set message content to be without the prefix + command
            false => return, // There was no prefix match, so there is no command to match.
        },
        None => return, // There is either no message content, or there is nothing to split at ' ' so we can return.
    };

    // We have a valid prefix! Let's check to see if the command is disabled.
    if framework.gateway.config.command_disabled(command) {
        disabled_command(&framework, command).await;
        return;
    }

    // Comand is allowed, try to match it
    match command {
        "uwu" => uwu::cmd(&framework).await,
        "hi" => hi::cmd(&framework).await,
        #[cfg(feature = "module-ai")]
        "ai" => super::ai::ai_handler_root(&framework).await,
        cmd => unknown_command(&framework, cmd).await,
    };
}

async fn unknown_command(framework: &MessageContext, cmd: &str) {
    let message_client = framework
        .gateway
        .twilight_client
        .create_message(framework.ctx.channel_id)
        .reply(framework.ctx.id);

    if let Err(why) = message_client
        .content(&format!(
            "Looks like you found a work in progress command. `{cmd} is not yet implemented.`"
        ))
        .await
    {
        tracing::error!(?why, "prefix_command - Failed to send message");
    }
}

async fn disabled_command(framework: &MessageContext, cmd: &str) {
    let message_client = framework
        .gateway
        .twilight_client
        .create_message(framework.ctx.channel_id)
        .reply(framework.ctx.id);

    if let Err(why) = message_client
        .content(&format!(
            "The command `{cmd}` is currently DISABLED! Sorry about that."
        ))
        .await
    {
        tracing::error!(?why, "prefix_command - Failed to send message");
    }
}
