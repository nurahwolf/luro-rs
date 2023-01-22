#![feature(let_chains)]

use events::{interaction_create::interaction_create, on_message::message, ready_listener::ready_listener};
use functions::deleted_message_formatted;
use luro_core::{Data, Error};
use luro_utilities::{alert_channel_defined, guild_accent_colour};
use poise::serenity_prelude::Context;
use tracing::{info, debug};

mod events;
mod functions;

/// **Luro's error handler**
///
/// This function is called every time we have an error. There are many types of errors, so we only handle the ones we are particularly interested in. The rest get forwarded to the default error handler.
pub async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {error:?}"),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
            ctx.send(|message| {
                message
                    .ephemeral(true)
                    .content(format!("Error in command `{}`: {:?}", ctx.command().name, error))
            })
            .await
            .expect("Could not send error to channel!");
        }
        // We are not interested in this particular error, so handle it by the built-in function.
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {e}")
            }
        }
    }
}

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
        poise::Event::ChannelCreate { channel } => {
            if let Some(alert_channel) = alert_channel_defined(&channel.guild_id, user_data, ctx).await {
                alert_channel
                    .send_message(ctx, |message| {
                        message.add_embed(|embed| {
                            embed
                                .title("Channel Created")
                                .description(format!("The channel {} just got created", channel.name()))
                                .color(guild_accent_colour(accent_colour, channel.guild(ctx)))
                        })
                    })
                    .await?;
                return Ok(());
            }
        }
        poise::Event::CategoryCreate { category } => {
            if let Some(alert_channel) = alert_channel_defined(&category.guild_id, user_data, ctx).await {
                alert_channel
                    .send_message(ctx, |message| {
                        message.add_embed(|embed| {
                            embed
                                .title("Category Created")
                                .description(format!("The category {} just got created", category.name()))
                                .color(guild_accent_colour(accent_colour, category.guild_id.to_guild_cached(ctx)))
                        })
                    })
                    .await?;
                return Ok(());
            }
        }
        poise::Event::CategoryDelete { category } => {
            if let Some(alert_channel) = alert_channel_defined(&category.guild_id, user_data, ctx).await {
                alert_channel
                    .send_message(ctx, |message| {
                        message.add_embed(|embed| {
                            embed
                                .title("Category Deleted")
                                .description(format!("The category {} just got DELETED!", category.name()))
                                .color(guild_accent_colour(accent_colour, category.guild_id.to_guild_cached(ctx)))
                        })
                    })
                    .await?;
                return Ok(());
            }
        }
        poise::Event::ChannelDelete { channel } => {
            if let Some(alert_channel) = alert_channel_defined(&channel.guild_id, user_data, ctx).await {
                alert_channel
                    .send_message(ctx, |message| {
                        message.add_embed(|embed| {
                            embed
                                .title("Channel Deleted")
                                .description(format!("The channel {} just got DELETED!", channel.name()))
                                .color(guild_accent_colour(accent_colour, channel.guild(ctx)))
                        })
                    })
                    .await?;
                return Ok(());
            }
        }
        poise::Event::ChannelPinsUpdate { pin } => {
            if let Some(guild_id) = pin.guild_id {
                if let Some(alert_channel) = alert_channel_defined(&&guild_id, user_data, ctx).await {
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
                if let Some(alert_channel) = alert_channel_defined(&guild_channel.guild_id, user_data, ctx).await {
                    alert_channel
                        .send_message(ctx, |message| {
                            message.add_embed(|embed| {
                                embed
                                    .title("Channel Updated")
                                    .description(format!("The channel {} just got updated!", guild_channel))
                                    .color(guild_accent_colour(accent_colour, guild_channel.guild(ctx)))
                            })
                        })
                        .await?;
                    return Ok(());
                }
            }
        }
        poise::Event::GuildBanAddition { guild_id, banned_user } => {
            if let Some(alert_channel) = alert_channel_defined(guild_id, user_data, ctx).await {
                alert_channel
                    .send_message(ctx, |message| {
                        message.add_embed(|embed| {
                            embed
                                .title("Member Banned")
                                .description(format!("The user {} ({}) just got banned!", banned_user, banned_user.id.0))
                                .color(guild_accent_colour(accent_colour, guild_id.to_guild_cached(ctx)))
                        })
                    })
                    .await?;
                return Ok(());
            }
        }
        poise::Event::GuildBanRemoval { guild_id, unbanned_user } => {
            if let Some(alert_channel) = alert_channel_defined(guild_id, user_data, ctx).await {
                alert_channel
                    .send_message(ctx, |message| {
                        message.add_embed(|embed| {
                            embed
                                .title("Member Unbanned")
                                .description(format!(
                                    "The user {} ({}) just got unbanned!",
                                    unbanned_user, unbanned_user.id.0
                                ))
                                .color(guild_accent_colour(accent_colour, guild_id.to_guild_cached(ctx)))
                        })
                    })
                    .await?;
                return Ok(());
            }
        }
        poise::Event::GuildCreate { guild, is_new: _ } => info!("Loaded guild {} ({}) into cache", guild.name, guild.id),
        // poise::Event::GuildDelete { incomplete, full } => todo!(),
        // poise::Event::GuildEmojisUpdate { guild_id, current_state } => todo!(),
        // poise::Event::GuildIntegrationsUpdate { guild_id } => todo!(),
        poise::Event::GuildMemberAddition { new_member } => {
            if let Some(alert_channel) = alert_channel_defined(&new_member.guild_id, user_data, ctx).await {
                alert_channel
                    .send_message(ctx, |message| {
                        message.add_embed(|embed| {
                            embed
                                .title("Member Joined")
                                .description(format!(
                                    "The user {} ({}) just joined the server!",
                                    new_member, new_member.user.id.0
                                ))
                                .color(guild_accent_colour(accent_colour, new_member.guild_id.to_guild_cached(ctx)))
                        })
                    })
                    .await?;
                return Ok(());
            }
        }
        poise::Event::GuildMemberRemoval {
            guild_id,
            user,
            member_data_if_available: _
        } => {
            if let Some(alert_channel) = alert_channel_defined(&guild_id, user_data, ctx).await {
                alert_channel
                    .send_message(ctx, |message| {
                        message.add_embed(|embed| {
                            embed
                                .title("Member Left")
                                .description(format!("The user {} ({}) just left the server!", user, user.id.0))
                                .color(guild_accent_colour(accent_colour, guild_id.to_guild_cached(ctx)))
                        })
                    })
                    .await?;
                return Ok(());
            }
        }
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
        poise::Event::InviteCreate { data } => {
            if let Some(guild_id) = data.guild_id {
                if let Some(alert_channel) = alert_channel_defined(&guild_id, user_data, ctx).await {
                    let description = match &data.inviter {
                        Some(inviter) => format!("The invite {} just got created by user {}!", data.code, inviter),
                        None => format!("The invite {} just got created by an unknown user!", data.code)
                    };
                    alert_channel
                        .send_message(ctx, |message| {
                            message.add_embed(|embed| {
                                embed
                                    .title("Invite Created")
                                    .description(format!("The invite {} just got created by user {}!", data.code, description))
                                    .color(guild_accent_colour(accent_colour, guild_id.to_guild_cached(ctx)))
                            })
                        })
                        .await?;
                    return Ok(());
                }
            }
        }
        poise::Event::InviteDelete { data } => {
            if let Some(guild_id) = data.guild_id {
                if let Some(alert_channel) = alert_channel_defined(&guild_id, user_data, ctx).await {
                    alert_channel
                        .send_message(ctx, |message| {
                            message.add_embed(|embed| {
                                embed
                                    .title("Invite Deleted")
                                    .description(format!("The invite {} just got created by an unknown user!", data.code))
                                    .color(guild_accent_colour(accent_colour, guild_id.to_guild_cached(ctx)))
                            })
                        })
                        .await?;
                    return Ok(());
                }
            }
        }
        poise::Event::MessageDelete {
            channel_id,
            deleted_message_id,
            guild_id
        } => {
            if let Some(guild_id) = guild_id {
                if let Some(alert_channel) = alert_channel_defined(guild_id, user_data, ctx).await {
                    let message =
                        deleted_message_formatted(ctx, &alert_channel, user_data, deleted_message_id.0, channel_id, false)
                            .await;

                    alert_channel
                        .send_message(ctx, |m| {
                            *m = message;
                            m
                        })
                        .await?;
                    return Ok(());
                }
            }
        }
        poise::Event::MessageDeleteBulk {
            channel_id,
            multiple_deleted_messages_ids,
            guild_id
        } => {
            if let Some(guild_id) = guild_id {
                if let Some(alert_channel) = alert_channel_defined(guild_id, user_data, ctx).await {
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
        } => {
            if let Some(guild_id) = event.guild_id {
                if let Some(alert_channel) = alert_channel_defined(&guild_id, user_data, ctx).await {
                    alert_channel
                        .send_message(ctx, |message| {
                            message.add_embed(|embed| {
                                embed.title("Message Edited");
                                if let Some(message_content) = &event.content {
                                    embed.description(message_content);
                                }

                                if let Some(new_message) = new {
                                    if new_message.content.is_empty() {
                                        embed.description("<THE NEW MESSAGE DOES NOT HAVE A BODY>");
                                    } else {
                                        embed.description(&new_message.content);
                                    };
                                    embed.url(new_message.link());
                                }

                                if let Some(old_message) = old_if_available {
                                    if old_message.content.is_empty() {
                                        embed.description("<THE OLD MESSAGE DOES NOT HAVE A BODY>");
                                    } else {
                                        embed.description(&old_message.content);
                                    }
                                }
                                embed.color(guild_accent_colour(accent_colour, guild_id.to_guild_cached(ctx)));
                                embed.field("Channel", event.channel_id, true);
                                if let Some(message_user) = &event.author {
                                    embed.field("User", message_user, true);
                                    embed.author(|author| {
                                        author
                                            .name(&message_user.name)
                                            .icon_url(&message_user.avatar_url().unwrap_or_default())
                                    });
                                };
                                embed
                            })
                        })
                        .await?;
                    return Ok(());
                }
            }
        }
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
