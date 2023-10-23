use luro_framework::{CommandInteraction, LuroCommand};
use serde::Serialize;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::RoleMarker, Id};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "modify_role",
    desc = "Modify a role while bypassing all other restrictions.",
    dm_permission = false
)]
pub struct ModifyRole {
    /// The role that should be assigned. It HAS to be below the bot for this to work.
    role: Id<RoleMarker>,
    /// If set, change the role position to where this one is
    position: Option<Id<RoleMarker>>,
    /// If set, change the role position to this exact number
    position_num: Option<i64>,
    /// A colour set. Pass either a HEXADECIMAL `0xDABEEF`, HEX `DABEEF` or number `1922942`.
    colour: Option<String>,
    /// A new name for a role if defined
    name: Option<String>,
}

// BUG: This is fucked currently: https://github.com/twilight-rs/twilight/issues/2209
#[derive(Serialize)]
struct Position {
    id: Id<RoleMarker>,
    position: i64,
}

impl LuroCommand for ModifyRole {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        // let (mut role_selected, mut role_position) = (None, None);
        let guild = match &ctx.guild {
            Some(guild) => guild,
            None => return ctx.response_simple(luro_framework::Response::NotGuild).await,
        };

        // for role in guild.roles.clone() {
        //     if role.id == self.role {
        //         role_selected = Some(role.clone());
        //     };

        //     // If self.position is defined
        //     if let Some(position) = self.position {
        //         if role.id == position {
        //             role_position = Some(role.clone());
        //         }
        //     }
        // }

        // if let Some(mut role_selected) = role_selected {
        //     let mut number = 1;
        //     let mut updated_role_list = Vec::new();
        //     let mut update_role = ctx.twilight_client.update_role(guild.id, role_selected.id);

        //     for role in guild.roles {
        //         info!(role.name);
        //         number += 1;
        //         let mut aaa = vec![(role.id, number)];
        //         updated_role_list.append(&mut aaa)
        //     }

        //     // If we are updating the position based on a previous role
        //     if let Some(role_position) = role_position {
        //         let positions: Vec<Position> = vec![Position {
        //             id: role_selected.id,
        //             position: role_position.position,
        //         }];
        //         let request = Request::builder(&Route::UpdateRolePositions { guild_id: guild.id.get() })
        //             .json(&positions)
        //             .build();
        //         ctx.twilight_client.request::<EmptyBody>(request?).await?;
        //     }

        //     // If we are updating the position based on an exact number
        //     if let Some(position) = self.position_num {
        //         let positions: Vec<Position> = vec![Position {
        //             id: role_selected.id,
        //             position,
        //         }];
        //         let request = Request::builder(&Route::UpdateRolePositions { guild_id: guild.id.get() })
        //             .json(&positions)
        //             .build();
        //         ctx.twilight_client.request::<EmptyBody>(request?).await?;
        //     }

        //     if let Some(ref name) = self.name {
        //         update_role = update_role.name(Some(name));
        //     }

        //     // If we are changing the colour
        //     if let Some(ref colour) = self.colour {
        //         let colour = if colour.starts_with("0x") {
        //             u32::from_str_radix(colour.as_str().strip_prefix("0x").unwrap(), 16)?
        //         } else if colour.chars().all(|char| char.is_ascii_hexdigit()) {
        //             u32::from_str_radix(colour.as_str(), 16)?
        //         } else {
        //             colour.parse::<u32>()?
        //         };

        //         role_selected.color = colour;

        //         if colour == 0 {
        //             update_role = update_role.color(None)
        //         } else {
        //             update_role = update_role.color(Some(colour))
        //         };
        //     }

        //     let updated_role = update_role.await?.model().await?;
        //     let mut embed = EmbedBuilder::default();
        //     let mut description = String::new();
        //     writeln!(description, "**Role:** <@&{0}> - {0}", updated_role.id)?;
        //     writeln!(description, "**Position:** {}", updated_role.position)?;
        //     write!(description, "**Permissons:**\n```{:?}```", updated_role.permissions)?;

        //     embed
        //         .title(updated_role.name)
        //         .description(description)
        //         .colour(ctx.accent_colour());
        //     if updated_role.color != 0 {
        //         embed.colour(role_selected.color);
        //     }
        //     if updated_role.hoist {
        //         embed.create_field("Hoisted", "True", true);
        //     }
        //     if updated_role.managed {
        //         embed.create_field("Managed", "True", true);
        //     }
        //     if updated_role.mentionable {
        //         embed.create_field("Mentionable", "True", true);
        //     }

        //     // TODO: Return an embed with new role information
        //     ctx.respond(|r| r.add_embed(embed).ephemeral()).await
        // } else {
        // TODO: Make this a response type
        ctx.respond(|r| r.content("No role found").ephemeral()).await
        // }
    }
}
