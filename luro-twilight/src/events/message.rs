use luro_framework::{Luro, LuroContext};
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn create(framework: LuroContext, event: Box<MessageCreate>) -> anyhow::Result<()> {
    if let Ok(user_characters) = framework.database.user_fetch_characters(event.author.id).await {
        let mut user_character = None;
        let first_word = event.content.split(' ').next().unwrap_or("");

        for character in user_characters {
            if let Some(prefix) = &character.prefix {
                if prefix == first_word {
                    user_character = Some(character);
                }
            }
        }

        if let Some(character) = user_character {
            let _ = framework.twilight_client.delete_message(event.channel_id, event.id).await; // We don't care if this fails
            let proxy_message = event.content.replace(first_word, "");
            let author = match &event.member {
                Some(member) => match event.guild_id {
                    Some(guild_id) => (event.author.clone(), member.clone(), guild_id).into(),
                    None => event.author.clone().into(),
                },
                None => event.author.clone().into(),
            };

            framework
                .proxy_character(
                    &author,
                    event.channel_id,
                    character,
                    proxy_message,
                    event.reference.as_ref(),
                    event.id,
                )
                .await?;
        }
    }

    Ok(())
}
