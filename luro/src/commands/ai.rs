use std::time::{Duration, SystemTime, UNIX_EPOCH};

use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use twilight_model::http::attachment::Attachment;
use twilight_util::builder::embed::EmbedBuilder;

use crate::models::message_context::MessageContext;

pub async fn ai_command_handler(framework: &MessageContext) {
    // Handle keyword commands if the bot is not mentioned
    match &framework.ctx.referenced_message {
        Some(referenced_message) => {
            if referenced_message.author.id == framework.gateway.current_user.id {
                ai_handler_root(framework).await;
            }
        }
        None => {
            if framework
                .ctx
                .mentions
                .iter()
                .any(|x| x.id == framework.gateway.current_user.id)
            {
                ai_handler_root(framework).await;
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    model: String,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
    images: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Response {
    created_at: String,
    done: bool,
    eval_count: Option<i64>,
    eval_duration: Option<i64>,
    load_duration: Option<i64>,
    message: Option<Message>,
    model: String,
    prompt_eval_count: Option<i64>,
    prompt_eval_duration: Option<i64>,
    total_duration: Option<i64>,
}

pub async fn ai_handler_root(framework: &MessageContext) {
    // Inform the user that we are actually doing something
    let twilight_message = match framework
        .gateway
        .twilight_client
        .create_message(framework.ctx.channel_id)
        .content("Generating response...")
        .reply(framework.ctx.id)
        .await
    {
        Ok(message_response) => match message_response.model().await {
            Ok(twilight_message) => twilight_message,
            Err(why) => {
                tracing::error!(?why, "ai_handler - Failed to create a message model");
                return;
            }
        },
        Err(why) => {
            tracing::error!(
                ?why,
                "ai_handler - Failed to send generating response message to an AI request"
            );
            return;
        }
    };

    let mut message_reference = framework.ctx.reference.clone();
    let mut messages = vec![];

    messages.push((
        framework.ctx.content.clone(),
        framework.ctx.author.id == framework.gateway.current_user.id,
    ));

    while let Some(ref message_ref) = message_reference {
        if let Some(message_id) = message_ref.message_id {
            if let Ok(message) = framework
                .gateway
                .twilight_client
                .message(framework.ctx.channel_id, message_id)
                .await
            {
                if let Ok(message) = message.model().await {
                    messages.push((
                        message.content.clone(),
                        message.author.id == framework.gateway.current_user.id,
                    ));

                    message_reference = message.reference;
                }
            }
        }
    }

    messages.reverse();
    tracing::info!("ai_handler - Total Messages: {}", messages.len());
    tracing::info!("ai_handler - {messages:#?}");

    if let Err(why) = ai_handler(framework, messages, twilight_message).await {
        if let Err(why) = framework
            .gateway
            .twilight_client
            .create_message(framework.ctx.channel_id)
            .content(&format!("AAAAAA YOU BROKE ME:\n```rs\n{why:?}```"))
            .await
        {
            tracing::error!(?why, "ai_handler - Failed to tell the user its fucked")
        }
        tracing::warn!(?why, "Failed to get response from AI handler")
    }
}

pub async fn ai_handler(
    // Gateway for interacting with the bot
    framework: &MessageContext,
    // The context for this invokation
    context: Vec<(String, bool)>,
    // The placeholder message, for updating
    mut twilight_message: twilight_model::channel::Message,
) -> anyhow::Result<()> {
    let mut response = Response::default();
    let mut chat_messages = vec![];

    for (content, bot) in context {
        chat_messages.push(Message {
            role: match bot {
                true => "assistant".into(),
                false => "user".into(),
            },
            content,
            images: None,
        })
    }

    let request = Request {
        // model: "dolphin-mixtral".into(),
        model: "wizard-vicuna-uncensored".into(),
        // model: "sira".into(),
        // model: "dolphin-phi".into(),
        messages: chat_messages,
    };

    let mut stream = framework
        .gateway
        .http_client
        .post("http://luro.local:11434/api/chat")
        // .post("http://localhost:11434/api/chat")
        .json(&request)
        .send()
        .await?
        .bytes_stream();

    while let Some(Ok(chunk)) = stream.next().await {
        tracing::debug!("Byte Chunk: {chunk:?}");

        match serde_json::from_slice::<Response>(&chunk[..]) {
            Ok(parsed_response) => {
                response.created_at = parsed_response.created_at;
                response.done = parsed_response.done;
                response.eval_count = parsed_response.eval_count;
                response.eval_duration = parsed_response.eval_duration;
                response.load_duration = parsed_response.load_duration;
                response.message = match response.message {
                    Some(mut message) => match parsed_response.message {
                        Some(parsed_message) => {
                            message.content.push_str(&parsed_message.content);
                            Some(message)
                        }
                        None => Some(message),
                    },
                    None => parsed_response.message,
                };
                response.model = parsed_response.model;
                response.prompt_eval_count = parsed_response.prompt_eval_count;
                response.prompt_eval_duration = parsed_response.prompt_eval_duration;
                response.total_duration = parsed_response.total_duration;
            }
            Err(why) => tracing::error!(?why, "Failed to parse response from ollama"),
        };

        // If the last twilight message was more than a second ago... Send an update to Discord.
        let twilight_time = twilight_message
            .edited_timestamp
            .unwrap_or(twilight_message.timestamp)
            .as_secs();
        let time_difference = SystemTime::now()
            .duration_since(UNIX_EPOCH + std::time::Duration::from_secs(twilight_time as u64))?;

        if time_difference > Duration::from_secs(3) {
            let content = response
                .message
                .as_ref()
                .map(|x| x.content.clone())
                .unwrap_or("Still generating response...".to_owned());
            update_message(framework, &mut twilight_message, &content).await;
        }
    }

    // Generated our full response, make sure the user has the final version
    let content = response
        .message
        .as_ref()
        .map(|x| x.content.clone())
        .unwrap_or("Still generating response...".to_owned());
    update_message(framework, &mut twilight_message, &content).await;

    Ok(())
}

/// Attempt to send a message update when some generation has been performed
async fn update_message(
    framework: &MessageContext,
    twilight_message: &mut twilight_model::channel::Message,
    mut message_update: &str,
) {
    if message_update.len() < 2 {
        message_update = "Still generating response...";
    }

    let message_client = framework
        .gateway
        .twilight_client
        .update_message(twilight_message.channel_id, twilight_message.id);
    let result = match message_update.len() {
        x if x < 2000 => message_client.content(Some(message_update)).await,
        x if x < 4096 => message_client.content(None).embeds(Some(&vec![EmbedBuilder::new().description(message_update).build()])).await,
        x => message_client
            .attachments(&[Attachment {
                description: None,
                file: message_update.as_bytes().to_vec(),
                filename: "result.txt".to_string(),
                id: 0
            }])
            .embeds(None)
            .content(Some(&format!("Wow, I created a response with `{x}` characters! You can find the response in the attached file."))).await,
    };

    // Check update was successful, if not try and send as a new message
    match result {
        Ok(message_update) => {
            if let Ok(message_update) = message_update.model().await {
                *twilight_message = message_update;
            };
        }
        Err(why) => {
            tracing::warn!(
                ?why,
                "update_message - failed to update message, attempting to send new message"
            );

            let message_client = framework
                .gateway
                .twilight_client
                .create_message(twilight_message.channel_id);
            let result = match message_update.len() {
                x if x < 2000 => message_client.content(message_update).await,
                x if x < 4096 => message_client.embeds(&vec![EmbedBuilder::new().description(message_update).build()]).await,
                x => message_client
                    .attachments(&[Attachment {
                        description: None,
                        file: message_update.as_bytes().to_vec(),
                        filename: "result.txt".to_string(),
                        id: 0
                    }])
                    .embeds(&vec![])
                    .content(&format!("Wow, I created a response with `{x}` characters! You can find th response in the attached file.")).await,
            };

            match result {
                Ok(message_update) => {
                    if let Ok(message_update) = message_update.model().await {
                        *twilight_message = message_update;
                    };
                }
                Err(why) => {
                    tracing::error!(?why, "update_message - failed to create message");
                }
            }
        }
    }
}
