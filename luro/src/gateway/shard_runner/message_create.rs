use twilight_gateway::MessageSender;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{
    gateway::{GatewayArc, GatewayResult},
    models::message_context::MessageContext,
};

pub async fn message_create_handler(
    gateway: GatewayArc,
    shard: MessageSender,
    event: Box<MessageCreate>,
) -> GatewayResult {
    let mut framework = MessageContext {
        gateway,
        shard,
        ctx: event,
    };

    #[cfg(feature = "module-ai")]
    crate::commands::ai_command_handler(&framework).await;
    #[cfg(feature = "module-character")]
    crate::commands::character_handler(&framework).await;
    #[cfg(feature = "module-keywords")]
    crate::commands::keyword_handler(&framework).await;
    #[cfg(feature = "module-prefix")]
    crate::commands::prefix_handler(&mut framework).await;

    Ok(())
}

// pub async fn create(framework: LuroContext, event: Box<MessageCreate>) -> anyhow::Result<()> {
//     if let Ok(user_characters) = framework.database.user_fetch_characters(event.author.id).await {
//         let mut user_character = None;
//         let first_word = event.content.split(' ').next().unwrap_or("");

//         for character in user_characters {
//             if let Some(prefix) = &character.prefix {
//                 if prefix == first_word {
//                     user_character = Some(character);
//                 }
//             }
//         }

//         if let Some(character) = user_character {
//             let _ = framework.twilight_client.delete_message(event.channel_id, event.id).await; // We don't care if this fails
//             let proxy_message = event.content.replace(first_word, "");
//             let author = match &event.member {
//                 Some(member) => match event.guild_id {
//                     Some(guild_id) => (event.author.clone(), member.clone(), guild_id).into(),
//                     None => event.author.clone().into(),
//                 },
//                 None => event.author.clone().into(),
//             };

//             framework
//                 .proxy_character(
//                     &author,
//                     event.channel_id,
//                     character,
//                     proxy_message,
//                     event.reference.as_ref(),
//                     event.id,
//                 )
//                 .await?;
//         }
//     }

//     Ok(())
// }
