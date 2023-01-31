use std::sync::Arc;

use luro_core::{quotes::{Quotes, Quote}, QUOTES_FILE_PATH};
use poise::serenity_prelude::RwLock;

pub async fn save_quote(quotes: Arc<RwLock<Quotes>>, mut new_quote: Vec<Quote>) {
    let quotes = &mut quotes.write().await;
    quotes.quotes.append(&mut new_quote);
    Quotes::write(quotes, QUOTES_FILE_PATH).await;

}