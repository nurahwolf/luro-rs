#![feature(let_chains)]
use luro_core::{Command, Context, Error, DATABASE_FILE_PATH};
use luro_utilities::guild_accent_colour;
use poise::serenity_prelude::{CacheHttp, CreateEmbed, Message};
use rkyv::{Archive, Deserialize, Serialize};
use zerocopy::AsBytes;

use crate::commands::{add, get, total};

mod commands;

/// Get some information on things, like guilds and users.
#[poise::command(slash_command, category = "DB", subcommands("get", "total", "add"))]
async fn sled(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This command only has subcommands I'm afraid :)").await?;
    Ok(())
}

pub fn sled_commands() -> [Command; 1] {
    [sled()]
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct LuroMessage {
    pub message_content: String,
    pub message_id: u64,
    pub channel_id: u64,
    pub user_id: u64,
    pub guild_id: Option<u64>
}

pub fn get_sled() -> std::result::Result<sled::Db, sled::Error> {
    sled::open(DATABASE_FILE_PATH)
}

pub fn add_discord_message(db: &sled::Db, message: Message) -> sled::Result<()> {
    let messages = if let Ok(messages) = db.open_tree(b"luromessage") {
        messages
    } else {
        panic!("Failed to get database messages");
    };
    let luro_message = if let Some(guild_id) = message.guild_id {
        LuroMessage {
            message_content: message.content,
            message_id: message.id.0,
            channel_id: message.channel_id.0,
            user_id: message.author.id.0,
            guild_id: Some(guild_id.0)
        }
    } else {
        LuroMessage {
            message_content: message.content,
            message_id: message.id.0,
            channel_id: message.channel_id.0,
            user_id: message.author.id.0,
            guild_id: None
        }
    };

    let bytes = match rkyv::to_bytes::<_, 1024>(&luro_message) {
        Ok(ok) => ok,
        Err(err) => panic!("DB: Failed to serialize: {err}")
    };

    if let Ok(_result) = messages.insert(message.id.0.as_bytes(), bytes.as_bytes()) {
    } else {
        panic!("Failed to insert");
    }

    Ok(())
}

pub fn get_discord_message(db: &sled::Db, id: u64) -> Result<LuroMessage, String> {
    let messages_tree = if let Ok(messages) = db.open_tree(b"luromessage") {
        messages
    } else {
        return Err("DB: Failed to open the database".to_string());
    };

    let messages_vec = match messages_tree.get(id.as_bytes()) {
        Ok(result) => result,
        Err(_) => return Err("DB: Failed to get anything from the database".to_string())
    };

    let messages_vec_resolved = match messages_vec {
        Some(result) => result,
        None => return Err("No message was returned from the database. Sure that is the right message ID?".to_string())
    };

    let luro_message = unsafe { rkyv::archived_root::<LuroMessage>(messages_vec_resolved.as_bytes()) };

    match luro_message.deserialize(&mut rkyv::Infallible) {
        Ok(luro_message) => return Ok(luro_message),
        Err(err) => return Err(format!("DB: Failed to deserialize: {err}"))
    };
}

pub fn total_messages_by_user(db: &sled::Db, user_id: u64) -> u64 {
    let messages = if let Ok(messages) = db.open_tree(b"luromessage") {
        messages
    } else {
        panic!("Failed to get database messages");
    };
    let mut total_messages = u64::default();

    for message in messages.iter().flatten() {
        let luro_message = unsafe { rkyv::archived_root::<LuroMessage>(message.1.as_bytes()) };

        if luro_message.user_id == user_id {
            total_messages += 1;
        }
    }
    total_messages
}

pub async fn get_message_formatted(ctx: Context<'_>, message_id: u64, hide: bool) -> Result<CreateEmbed, Error> {
    let accent_colour = ctx.data().config.read().await.accent_colour;
    let luro_message = match get_discord_message(&ctx.data().database, message_id) {
        Ok(ok) => ok,
        Err(err) => {
            return Err(err.into());
        }
    };
    let message_resolved = ctx
        .serenity_context()
        .http
        .get_message(luro_message.channel_id, luro_message.message_id)
        .await;
    let mut embed = CreateEmbed::default();

    embed.description(&luro_message.message_content);
    embed.color(guild_accent_colour(accent_colour, ctx.guild()));
    embed.footer(|footer| footer.text("This message was fetched from the database, so most likely no longer exists"));

    if let Ok(message_user) = ctx.http().get_user(luro_message.user_id).await {
        embed.author(|author| {
            author
                .name(&message_user.name)
                .icon_url(&message_user.avatar_url().unwrap_or_default())
        });
    }

    if !hide {
        embed.field("Message ID", &luro_message.message_id, true);
        embed.field("Channel ID", &luro_message.channel_id, true);
        embed.field("User ID", &luro_message.user_id, true);

        if let Some(guild_id) = &luro_message.guild_id && message_resolved.is_err() {
            embed.field("Guild ID", guild_id, true);
        }
    }

    if let Ok(message_resolved) = message_resolved {
        embed.footer(|footer| footer.text("This message was fully resolved, so it still exists in Discord"));
        embed.author(|author| {
            author
                .name(&message_resolved.author.name)
                .icon_url(&message_resolved.author.avatar_url().unwrap_or_default())
                .url(&message_resolved.link())
        });

        if let Some(guild) = message_resolved.guild(ctx) {
            embed.footer(|footer| {
                footer.icon_url(guild.icon_url().unwrap_or_default()).text(format!(
                    "{} - This message was fully resolved, so it still exists in Discord",
                    guild.name
                ))
            });
        } else {
            if let Some(guild_id) = &luro_message.guild_id && !hide {
                embed.field("Guild ID", guild_id, true);
            }
        }
    };
    Ok(embed)
}
