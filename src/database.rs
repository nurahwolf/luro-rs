use poise::serenity_prelude::Message;
use rkyv::{Archive, Deserialize, Serialize};
use zerocopy::AsBytes;

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
pub struct LuroMessage {
    pub message_content: String,
    pub message_id: u64,
    pub channel_id: u64,
    pub user_id: u64,
    pub guild_id: Option<u64>
}

pub fn add_discord_message(db: &sled::Db, message: Message) -> sled::Result<()> {
    let messages = if let Ok(messages) = db.open_tree(b"luromessage:") {
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

    let bytes = rkyv::to_bytes::<_, 256>(&luro_message).unwrap();

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
        Err(_) => todo!()
    };

    let messages_vec_resolved = match messages_vec {
        Some(result) => result,
        None => todo!()
    };

    let luro_message = unsafe { rkyv::archived_root::<LuroMessage>(messages_vec_resolved.as_bytes()) };

    let deserialized: LuroMessage = luro_message.deserialize(&mut rkyv::Infallible).unwrap();
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
        let deserialized: LuroMessage = luro_message.deserialize(&mut rkyv::Infallible).unwrap();

        if deserialized.user_id == user_id {
            total_messages += 1;
        }
    }
    total_messages
}
