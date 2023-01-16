use luro_core::{DATABASE_FILE_PATH, Context, Error, Command};
use poise::serenity_prelude::Message;
use rkyv::{Archive, Deserialize, Serialize};
use zerocopy::AsBytes;

use crate::commands::{get, total, add};

mod commands;

/// Get some information on things, like guilds and users.
#[poise::command(
    slash_command,
    category = "DB",
    subcommands("get", "total", "add")
)]
async fn sled(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This command only has subcommands I'm afraid :)").await?;
    Ok(())
}

pub fn sled_commands() -> [Command; 1] {
    [
        sled(),
    ]
}

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct LuroMessage {
    pub message_content: String,
    pub message_id: u64,
    pub channel_id: u64,
    pub user_id: u64,
    pub guild_id: Option<u64>
}

pub fn get_sled() -> std::result::Result<sled::Db, sled::Error>  {
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

pub fn get_discord_message(db: &sled::Db, id: u64) -> LuroMessage {
    let messages_tree = if let Ok(messages) = db.open_tree(b"luromessage") {
        messages
    } else {
        panic!("Failed to get database messages");
    };

    let messages_vec = match messages_tree.get(id.as_bytes()) {
        Ok(result) => result,
        Err(_) => panic!("Failed to find anything with that key")
    };

    let messages_vec_resolved = match messages_vec {
        Some(result) => result,
        None => panic!("Failed to resolve the vec for that key")
    };

    let luro_message = unsafe { rkyv::archived_root::<LuroMessage>(messages_vec_resolved.as_bytes()) };

    let deserialized: LuroMessage = match luro_message.deserialize(&mut rkyv::Infallible) {
        Ok(ok) => ok,
        Err(err) => panic!("DB: Failed to deserialize: {err}")
    };
    deserialized
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
