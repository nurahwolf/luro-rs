use std::sync::Arc;

use anyhow::Error;
use futures::Future;
use tracing::info;
use twilight_gateway::{stream::ShardRef, Event};
use twilight_model::{application::interaction::InteractionData, id::Id};
use twilight_util::builder::embed::EmbedBuilder;

use crate::{commands, event_handler::ready::ready_listener, luro::Luro};
mod ready;

impl Luro {
    pub async fn handle_event(
        self: Arc<Self>,
        event: Event,
        shard: ShardRef<'_>,
    ) -> Result<(), Error> {
        self.twilight_cache.update(&event);
        self.twilight_standby.process(&event);
        self.lavalink.process(&event).await?;

        match event {
            Event::Ready(ready) => {
                info!("Handling Event");
                spawn(send_embed(
                    self.clone(),
                    "Ready".to_string(),
                    format!("Luro is now ready: {:?}", ready),
                ));
                spawn(ready_listener(self, ready));
            }
            Event::InteractionCreate(interaction) => match &interaction.data {
                Some(InteractionData::ApplicationCommand(command)) => {
                    if let (Some(channel), Some(user)) =
                        (interaction.channel_id, interaction.user.clone())
                    {
                        tracing::info!(
                            "{} command in channel {} by {}",
                            command.name,
                            channel,
                            user.name
                        );
                    };

                    commands::handle_command(&self, &interaction, command, shard).await;
                }
                Some(InteractionData::MessageComponent(component)) => {
                    commands::handle_component(&self, &interaction, component).await;
                }
                _ => {}
            },
            Event::AutoModerationActionExecution(_) => todo!(),
            Event::AutoModerationRuleCreate(_) => todo!(),
            Event::AutoModerationRuleDelete(_) => todo!(),
            Event::AutoModerationRuleUpdate(_) => todo!(),
            Event::BanAdd(_) => todo!(),
            Event::BanRemove(_) => todo!(),
            Event::ChannelCreate(_) => todo!(),
            Event::ChannelDelete(_) => todo!(),
            Event::ChannelPinsUpdate(_) => todo!(),
            Event::ChannelUpdate(_) => todo!(),
            Event::CommandPermissionsUpdate(_) => todo!(),
            Event::GatewayClose(_) => todo!(),
            Event::GatewayHeartbeat(heartbeat) => {
                send_embed(
                    self,
                    "Gateway Heartbeat".to_string(),
                    format!("Heartbeat: {}", heartbeat),
                )
                .await?
            }
            Event::GatewayHeartbeatAck => {
                info!("Gateway Heartbeat Acknowledged")
            }
            Event::GatewayHello(hello) => {
                send_embed(
                    self,
                    "Gateway Hello".to_string(),
                    format!("Heartbeat: {}", hello.heartbeat_interval),
                )
                .await?
            }
            Event::GatewayInvalidateSession(_) => todo!(),
            Event::GatewayReconnect => todo!(),
            Event::GiftCodeUpdate => todo!(),
            Event::GuildAuditLogEntryCreate(_) => todo!(),
            Event::GuildCreate(_) => todo!(),
            Event::GuildDelete(_) => todo!(),
            Event::GuildEmojisUpdate(_) => todo!(),
            Event::GuildIntegrationsUpdate(_) => todo!(),
            Event::GuildScheduledEventCreate(_) => todo!(),
            Event::GuildScheduledEventDelete(_) => todo!(),
            Event::GuildScheduledEventUpdate(_) => todo!(),
            Event::GuildScheduledEventUserAdd(_) => todo!(),
            Event::GuildScheduledEventUserRemove(_) => todo!(),
            Event::GuildStickersUpdate(_) => todo!(),
            Event::GuildUpdate(guild) => {
                spawn(send_embed(
                    self.clone(),
                    "Guild Update".to_string(),
                    format!("Guild: {}", guild.name),
                ));
            }
            Event::IntegrationCreate(_) => todo!(),
            Event::IntegrationDelete(_) => todo!(),
            Event::IntegrationUpdate(_) => todo!(),
            Event::InviteCreate(invite) => {
                info!("An invite was created!");
                spawn(send_embed(
                    self,
                    "Invite Created".to_string(),
                    format!("Invite: {}", invite.code),
                ));
            }
            Event::InviteDelete(invite) => {
                info!("An invite was deleted!");
                spawn(send_embed(
                    self,
                    "Invite Deleted".to_string(),
                    format!("Invite: {}", invite.code),
                ));
            }
            Event::MemberAdd(member) => {
                spawn(send_embed(
                    self,
                    "Member Joined".to_string(),
                    format!("Member: {}", member.user.name),
                ));
            }
            Event::MemberRemove(member) => {
                spawn(send_embed(
                    self,
                    "Member Removed".to_string(),
                    format!("Member: {}", member.user.name),
                ));
            }
            Event::MemberUpdate(_) => todo!(),
            Event::MemberChunk(_) => todo!(),
            Event::MessageCreate(message) => {
                if message.author.id != Id::new(180285980232646656) || !message.author.bot {
                    spawn(send_embed(
                        self.clone(),
                        "Message Create".to_string(),
                        format!(
                            "Author: {}\nContent: {}",
                            message.author.name, message.0.content
                        ),
                    ));
                }
            }
            Event::MessageDelete(message) => {
                spawn(send_embed(
                    self.clone(),
                    "Message Delete".to_string(),
                    format!(
                        "Channel ID: <#{}>\nMessage ID: {}",
                        message.channel_id, message.id
                    ),
                ));
            }
            Event::MessageDeleteBulk(messages) => {
                spawn(send_embed(
                    self.clone(),
                    "Bulk Message Delete".to_string(),
                    format!(
                        "Channel ID: <#{}>\nTotal: {}",
                        messages.channel_id,
                        messages.ids.len()
                    ),
                ));
            }
            Event::MessageUpdate(message) => {
                let content = match message.content {
                    Some(content) => content,
                    None => "No Content Available".to_string(),
                };
                spawn(send_embed(
                    self.clone(),
                    "Message Updated".to_string(),
                    format!(
                        "Channel ID: <#{}>\nContent: {}",
                        message.channel_id, content
                    ),
                ));
            }
            Event::PresenceUpdate(_) => todo!(),
            Event::ReactionAdd(_) => todo!(),
            Event::ReactionRemove(_) => todo!(),
            Event::ReactionRemoveAll(_) => todo!(),
            Event::ReactionRemoveEmoji(_) => todo!(),
            Event::Resumed => {
                spawn(send_embed(
                    self.clone(),
                    "Shard Resumed".to_string(),
                    "A shard has resumed".to_string(),
                ));
            }
            Event::RoleCreate(role) => {
                spawn(send_embed(
                    self.clone(),
                    "Role Created".to_string(),
                    format!("The role {} was created", role.role.name),
                ));
            }
            Event::RoleDelete(_) => todo!(),
            Event::RoleUpdate(_) => todo!(),
            Event::StageInstanceCreate(_) => todo!(),
            Event::StageInstanceDelete(_) => todo!(),
            Event::StageInstanceUpdate(_) => todo!(),
            Event::ThreadCreate(thread) => {
                spawn(send_embed(
                    self.clone(),
                    "Thread Created".to_string(),
                    format!("The thread <#{}> was created", thread.0.id),
                ));
            }
            Event::ThreadDelete(_) => todo!(),
            Event::ThreadListSync(_) => todo!(),
            Event::ThreadMemberUpdate(_) => todo!(),
            Event::ThreadMembersUpdate(_) => todo!(),
            Event::ThreadUpdate(thread) => {
                spawn(send_embed(
                    self.clone(),
                    "Thread Updated".to_string(),
                    format!("The thread <#{}> was updated", thread.0.id),
                ));
            }
            Event::TypingStart(_) => todo!(),
            Event::UnavailableGuild(_) => todo!(),
            Event::UserUpdate(user) => {
                spawn(send_embed(
                    self.clone(),
                    "User Updated".to_string(),
                    format!("The user {} was updated", user.name),
                ));
            }
            Event::VoiceServerUpdate(voice) => {
                spawn(send_embed(
                    self.clone(),
                    "Voice Server Updated".to_string(),
                    format!("The voice interaction {} was updated", voice.token),
                ));
            }
            Event::VoiceStateUpdate(voice) => {
                spawn(send_embed(
                    self.clone(),
                    "Voice State Updated".to_string(),
                    format!("The user <@{}> was updated", voice.user_id),
                ));
            }
            Event::WebhooksUpdate(webhook) => {
                send_embed(
                    self,
                    "Webhook Update".to_string(),
                    format!(
                        "The webhook in channel {} - guild {} was updated",
                        webhook.channel_id, webhook.guild_id
                    ),
                )
                .await?
            }
            _ => {}
        }

        Ok(())
    }
}

fn spawn(future: impl Future<Output = anyhow::Result<()>> + Send + 'static) {
    tokio::spawn(async move {
        if let Err(why) = future.await {
            tracing::warn!("handler error: {why:?}");
        }
    });
}

pub async fn send_embed(luro: Arc<Luro>, title: String, description: String) -> Result<(), Error> {
    let embed = EmbedBuilder::default()
        .title(title)
        .description(description)
        .color(0xDABEEF)
        .build();
    let _message = luro
        .twilight_client
        .create_message(Id::new(1066690358588743760))
        .embeds(&[embed])?
        .await?;
    Ok(())
}
