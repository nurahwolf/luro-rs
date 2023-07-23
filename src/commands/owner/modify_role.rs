use async_trait::async_trait;
use serde::Serialize;
use std::fmt::Write;
use tracing::info;
use twilight_gateway::MessageSender;
use twilight_http::{request::Request, response::marker::EmptyBody, routing::Route};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::Interaction,
    id::{marker::RoleMarker, Id}
};
use twilight_util::builder::embed::EmbedFieldBuilder;

use crate::{
    interactions::InteractionResponse, models::LuroResponse, responses::not_guild::not_guild_response, LuroContext,
    SlashResponse
};

use super::LuroCommand;
#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "modify_role",
    desc = "Modify a role while bypassing all other restrictions.",
    dm_permission = false
)]
pub struct ModifyRoleCommand {
    /// The role that should be assigned. It HAS to be below the bot for this to work.
    role: Id<RoleMarker>,
    /// If set, change the role position to where this one is
    position: Option<Id<RoleMarker>>,
    /// If set, change the role position to this exact number
    position_num: Option<i64>,
    /// A colour set. Pass either a HEXADECIMAL `0xDABEEF`, HEX `DABEEF` or number `1922942`.
    pub colour: Option<String>
}

// BUG: This is fucked currently: https://github.com/twilight-rs/twilight/issues/2209
#[derive(Serialize)]
struct Position {
    id: Id<RoleMarker>,
    position: i64
}

#[async_trait]
impl LuroCommand for ModifyRoleCommand {
    async fn run_command(self, interaction: Interaction, ctx: LuroContext, _shard: MessageSender) -> SlashResponse {
        let luro_response = LuroResponse {
            ephemeral: false,
            deferred: false
        };
        let (_, _, _) = self.interaction_context(&interaction, "owner modify_role")?;
        let (mut role_selected, mut role_position) = (None, None);

        // Guild to modify
        let guild = ctx
            .twilight_client
            .guild(match interaction.guild_id {
                Some(guild_id) => guild_id,
                None => return Ok(not_guild_response(luro_response))
            })
            .await?
            .model()
            .await?;

        for role in guild.roles.clone() {
            if role.id == self.role {
                role_selected = Some(role.clone());
            };

            // If self.position is defined
            if let Some(position) = self.position {
                if role.id == position {
                    role_position = Some(role.clone());
                }
            }
        }

        if let Some(mut role_selected) = role_selected {
            let mut number = 1;
            let mut updated_role_list = Vec::new();

            for role in guild.roles {
                info!(role.name);
                number += 1;
                let mut aaa = vec![(role.id, number)];
                updated_role_list.append(&mut aaa)
            }

            // If we are updating the position based on a previous role
            if let Some(role_position) = role_position {
                let positions: Vec<Position> = vec![Position {
                    id: role_selected.id,
                    position: role_position.position
                }];
                let request = Request::builder(&Route::UpdateRolePositions {
                    guild_id: guild.id.get()
                })
                .json(&positions)?
                .build();
                ctx.twilight_client.request::<EmptyBody>(request).await?;
            }

            // If we are updating the position based on an exact number
            if let Some(position) = self.position_num {
                let positions: Vec<Position> = vec![Position {
                    id: role_selected.id,
                    position
                }];
                let request = Request::builder(&Route::UpdateRolePositions {
                    guild_id: guild.id.get()
                })
                .json(&positions)?
                .build();
                ctx.twilight_client.request::<EmptyBody>(request).await?;
            }

            // If we are changing the colour
            if let Some(ref colour) = self.colour {
                let colour = if colour.starts_with("0x") {
                    u32::from_str_radix(colour.as_str().strip_prefix("0x").unwrap(), 16)?
                } else if colour.chars().all(|char| char.is_ascii_hexdigit()) {
                    u32::from_str_radix(colour.as_str(), 16)?
                } else {
                    colour.parse::<u32>()?
                };

                role_selected.color = colour;

                if colour == 0 {
                    ctx.twilight_client
                        .update_role(guild.id, role_selected.id)
                        .color(None)
                        .await?;
                } else {
                    ctx.twilight_client
                        .update_role(guild.id, role_selected.id)
                        .color(Some(colour))
                        .await?;
                };
            }

            let mut embed = self.default_embed(&ctx, Some(guild.id));
            let mut description = String::new();
            writeln!(description, "**Role:** <@&{0}> - {0}", role_selected.id)?;
            writeln!(description, "**Position:** {}", role_selected.position)?;
            write!(description, "**Permissons:**\n```{:?}```", role_selected.permissions)?;

            embed = embed.title(role_selected.name);
            embed = embed.description(description);
            if role_selected.color != 0 {
                embed = embed.color(role_selected.color);
            }
            if role_selected.hoist {
                embed = embed.field(EmbedFieldBuilder::new("Hoisted", "True").inline())
            }
            if role_selected.managed {
                embed = embed.field(EmbedFieldBuilder::new("Managed", "True").inline())
            }
            if role_selected.mentionable {
                embed = embed.field(EmbedFieldBuilder::new("Mentionable", "True").inline())
            }

            // TODO: Return an embed with new role information
            Ok(InteractionResponse::Embed {
                embeds: vec![embed.build()],
                luro_response
            })
        } else {
            // TODO: Make this a response type
            Ok(InteractionResponse::Content {
                content: "No role found".to_owned(),
                luro_response
            })
        }
    }
}
