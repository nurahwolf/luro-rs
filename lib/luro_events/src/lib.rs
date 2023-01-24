#![feature(let_chains)]

use events::{
    category_created::category_create, category_deleted::category_delete, channel_create::channel_create,
    channel_delete::channel_delete, interaction_create::interaction_create, invite_created::invite_create,
    invite_deleted::invite_deleted, member_banned::member_banned, member_joined::member_joined, member_left::member_left,
    member_unbanned::member_unbanned, message_deleted::message_deleted, message_updated::message_updated, on_message::message,
    ready_listener::ready_listener
};
use luro_core::{Data, Error};
use luro_utilities::{discod_event_log_channel_defined, guild_accent_colour, moderator_actions_log_channel_defined};
use poise::serenity_prelude::Context;
use tracing::{debug, info};

mod events;
pub mod on_error;

/// **Luro's event listener**
///
/// This function is called every time Discord pushes an event, which is then matched and reacted to accordingly.
pub async fn event_listener(
    ctx: &Context,
    event: &poise::Event<'_>,
    framework: poise::FrameworkContext<'_, Data, Error>,
    user_data: &Data
) -> Result<(), Error> {
    let accent_colour = user_data.config.read().await.accent_colour;

    match event {
        poise::Event::Ready { data_about_bot } => ready_listener(data_about_bot, ctx).await?,
        poise::Event::InteractionCreate { interaction } => interaction_create(interaction).await?,
        poise::Event::Message { new_message } => message(new_message, ctx, &framework, user_data).await?,
        poise::Event::CacheReady { guilds: _ } => info!("Luro's cache is now ready!"),
        poise::Event::ChannelCreate { channel } => channel_create(ctx, user_data, accent_colour, channel).await?,
        poise::Event::CategoryCreate { category } => category_create(ctx, user_data, accent_colour, category).await?,
        poise::Event::CategoryDelete { category } => category_delete(ctx, user_data, accent_colour, category).await?,
        poise::Event::ChannelDelete { channel } => channel_delete(ctx, user_data, accent_colour, channel).await?,
        poise::Event::ChannelPinsUpdate { pin } => {
            if let Some(guild_id) = pin.guild_id {
                if let Some(alert_channel) = discod_event_log_channel_defined(&guild_id, user_data, ctx).await {
                    alert_channel
                        .send_message(ctx, |message| {
                            message.add_embed(|embed| {
                                embed
                                    .title("Pins Updated")
                                    .description(format!("The pins in {} just got updated!", pin.channel_id))
                                    .color(guild_accent_colour(accent_colour, guild_id.to_guild_cached(ctx)))
                            })
                        })
                        .await?;
                    return Ok(());
                }
            }
        }
        poise::Event::ChannelUpdate { old: _, new } => {
            if let Some(guild_channel) = new.clone().guild() {
                if let Some(alert_channel) = discod_event_log_channel_defined(&guild_channel.guild_id, user_data, ctx).await {
                    alert_channel
                        .send_message(ctx, |message| {
                            message.add_embed(|embed| {
                                embed
                                    .title("Channel Updated")
                                    .description(format!("The channel {guild_channel} just got updated!"))
                                    .color(guild_accent_colour(accent_colour, guild_channel.guild(ctx)))
                            })
                        })
                        .await?;
                    return Ok(());
                }
            }
        }
        poise::Event::GuildBanAddition { guild_id, banned_user } => {
            member_banned(ctx, user_data, accent_colour, guild_id, banned_user).await?
        }
        poise::Event::GuildBanRemoval { guild_id, unbanned_user } => {
            member_unbanned(ctx, user_data, accent_colour, guild_id, unbanned_user).await?
        }
        poise::Event::GuildCreate { guild, is_new: _ } => info!("Loaded guild {} ({}) into cache", guild.name, guild.id),
        // poise::Event::GuildDelete { incomplete, full } => todo!(),
        // poise::Event::GuildEmojisUpdate { guild_id, current_state } => todo!(),
        // poise::Event::GuildIntegrationsUpdate { guild_id } => todo!(),
        poise::Event::GuildMemberAddition { new_member } => member_joined(ctx, user_data, accent_colour, new_member).await?,
        poise::Event::GuildMemberRemoval {
            guild_id,
            user,
            member_data_if_available: _
        } => member_left(ctx, user_data, accent_colour, guild_id, user).await?,
        // poise::Event::GuildMembersChunk { chunk } => todo!(),
        // poise::Event::GuildRoleCreate { new } => todo!(),
        // poise::Event::GuildRoleDelete { guild_id, removed_role_id, removed_role_data_if_available } => todo!(),
        // poise::Event::GuildRoleUpdate { old_data_if_available, new } => todo!(),
        // poise::Event::GuildStickersUpdate { guild_id, current_state } => todo!(),
        // poise::Event::GuildUnavailable { guild_id } => todo!(),
        // poise::Event::GuildUpdate { old_data_if_available, new_but_incomplete } => todo!(),
        // poise::Event::IntegrationCreate { integration } => todo!(),
        // poise::Event::IntegrationUpdate { integration } => todo!(),
        // poise::Event::IntegrationDelete { integration_id, guild_id, application_id } => todo!(),
        poise::Event::InviteCreate { data } => invite_create(ctx, user_data, accent_colour, data).await?,
        poise::Event::InviteDelete { data } => invite_deleted(ctx, user_data, accent_colour, data).await?,
        poise::Event::MessageDelete {
            channel_id,
            deleted_message_id,
            guild_id
        } => message_deleted(ctx, user_data, accent_colour, channel_id, deleted_message_id, guild_id).await?,
        poise::Event::MessageDeleteBulk {
            channel_id,
            multiple_deleted_messages_ids,
            guild_id
        } => {
            if let Some(guild_id) = guild_id {
                if let Some(alert_channel) = moderator_actions_log_channel_defined(guild_id, user_data, ctx).await {
                    alert_channel
                        .send_message(ctx, |message| {
                            message.add_embed(|embed| {
                                embed
                                    .title("Bulk Messages Deleted")
                                    .description(format!(
                                        "A total of {} just got deleted in channel {}!",
                                        multiple_deleted_messages_ids.len(),
                                        channel_id
                                    ))
                                    .color(guild_accent_colour(accent_colour, guild_id.to_guild_cached(ctx)))
                            })
                        })
                        .await?;
                    return Ok(());
                }
            }
        }
        poise::Event::MessageUpdate {
            old_if_available,
            new,
            event
        } => message_updated(ctx, user_data, accent_colour, old_if_available, new, event).await?,
        // poise::Event::ReactionAdd { add_reaction } => todo!(),
        // poise::Event::ReactionRemove { removed_reaction } => todo!(),
        // poise::Event::ReactionRemoveAll { channel_id, removed_from_message_id } => todo!(),
        // poise::Event::PresenceReplace { new_presences } => todo!(),
        // poise::Event::Resume { event } => todo!(),
        // poise::Event::ShardStageUpdate { update } => todo!(),
        // poise::Event::StageInstanceCreate { stage_instance } => todo!(),
        // poise::Event::StageInstanceDelete { stage_instance } => todo!(),
        // poise::Event::StageInstanceUpdate { stage_instance } => todo!(),
        // poise::Event::ThreadCreate { thread } => todo!(),
        // poise::Event::ThreadDelete { thread } => todo!(),
        // poise::Event::ThreadListSync { thread_list_sync } => todo!(),
        // poise::Event::ThreadMemberUpdate { thread_member } => todo!(),
        // poise::Event::ThreadMembersUpdate { thread_members_update } => todo!(),
        // poise::Event::ThreadUpdate { thread } => todo!(),
        poise::Event::Unknown { name, raw } => debug!("Got an unknown event {}: {:?}", name, raw),
        // poise::Event::UserUpdate { old_data, new } => todo!(),
        // poise::Event::VoiceServerUpdate { update } => todo!(),
        // poise::Event::VoiceStateUpdate { old, new } => todo!(),
        // poise::Event::WebhookUpdate { guild_id, belongs_to_channel_id } => todo!(),
        poise::Event::PresenceUpdate { new_data: _ } => {} // Ignore this event
        poise::Event::TypingStart { event: _ } => {}       // Ignore this event
        poise::Event::GuildMemberUpdate {
            old_if_available: _,
            new: _
        } => {} // Ignore this event

        _ => {
            info!("Got an event in listener: {:?}", event.name());
        }
    }

    Ok(())
}
