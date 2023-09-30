use std::collections::HashMap;

use futures_util::TryStreamExt;
use luro_model::message::LuroMessage;
use sqlx::Error;
use twilight_model::id::{marker::MessageMarker, Id};

use crate::{DatabaseMessage, LuroDatabase};

impl DatabaseMessage {
    pub fn luro_message(&self) -> LuroMessage {
        LuroMessage {
            author: Id::new(self.author_id as u64),
            id: Id::new(self.message_id as u64),
            content: self.content.clone().unwrap_or_default(),
            ..Default::default()
        }
    }
}

impl LuroDatabase {
    pub async fn get_message(&self, id: i64) -> Result<Option<LuroMessage>, Error> {
        let query = sqlx::query_as!(
            DatabaseMessage,
            "SELECT message_id, author_id, content FROM messages WHERE message_id = $1",
            id
        );

        query.fetch_optional(&self.0).await.map(|x| x.map(|x| x.luro_message()))
    }

    pub async fn get_messages(&self) -> HashMap<Id<MessageMarker>, LuroMessage> {
        let mut messages = HashMap::new();
        let mut query = sqlx::query_as!(DatabaseMessage, "SELECT message_id, author_id, content FROM messages",).fetch(&self.0);

        while let Ok(Some(message)) = query.try_next().await {
            messages.insert(Id::new(message.author_id as u64), message.luro_message());
        }

        messages
    }

    pub async fn update_message(&self, message: impl Into<LuroMessage>) -> Result<LuroMessage, Error> {
        let message = message.into();
        let query = sqlx::query_as!(
            DatabaseMessage,
            "INSERT INTO messages (message_id, author_id, content) VALUES ($1, $2, $3) ON CONFLICT (message_id) DO UPDATE SET author_id = $2, content = $3 RETURNING message_id, author_id, content",
            message.id.get() as i64,
            message.author.get() as i64,
            message.content
        );

        query.fetch_one(&self.0).await.map(|x| x.luro_message())
    }
}
