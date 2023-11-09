use luro_framework::{CommandInteraction, Luro, LuroCommand};
use rand::{seq::SliceRandom, thread_rng};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{http::attachment::Attachment, id::Id};

use super::{buttons, guild_information, user_information};

const REMARK: [&str; 3] = ["Hey <user>!", "Great to see ya, <user>!", "Whoa, it's <user>!"];

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "user", desc = "Information about a user")]
pub struct InfoUser {
    /// The user to get, gets yourself if not specified
    pub user: Option<ResolvedUser>,
    /// Optionally try to get a user from a different guild
    guild: Option<i64>,
    /// Hide the user's avatar thumbnail. Still shows it in the author field!
    hide_avatar: Option<bool>,
    /// Set this if you want a copy of your data.
    gdpr_export: Option<bool>,
    /// Set this to true to get fresh data, if for some reason your profile is out of date
    sync: Option<bool>,
}

impl LuroCommand for InfoUser {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let description = REMARK.choose(&mut thread_rng()).unwrap_or(&REMARK[0]);
        let mut embed = ctx.default_embed().await;
        let mut user = ctx.get_specified_user_or_author(self.user.as_ref()).await?;

        if let Some(guild_id) = self.guild.map(|x| Id::new(x as u64)) {
            user = ctx.fetch_member_only(user.user_id, guild_id).await?
        }

        if self.sync.unwrap_or_default() {
            ctx.database.user_sync(&mut user).await;
        }

        embed
            .author(|author| {
                author
                    .name(format!("Infomation on {} | {}", user.name, user.user_id))
                    .icon_url(user.avatar_url())
            })
            .description(description.replace("<user>", &format!("<@{}>", user.user_id)))
            .footer(|f| {
                f.text(if user.member.is_some() && user.data.is_some() {
                    "Luro Member - Data fetched from my database, including guild information!"
                } else if user.member.is_some() {
                    "Twilight Member - Data fetched using the Discord API, including guild data"
                } else if user.data.is_some() {
                    "Luro User - Data fetched from my database only, with includes your custom stuff!"
                } else {
                    "Twilight User - Data fetched using the Discord API"
                })
            });

        if !self.hide_avatar.unwrap_or_default() {
            embed.thumbnail(|thumbnail| thumbnail.url(user.avatar_url()));
        }

        if let Some(ref banner) = user.banner_url() {
            embed.image(|i| i.url(banner));
        }

        if let Some(accent_colour) = user.accent_colour {
            embed.colour(accent_colour);
        } else if let Some(member) = &user.member {
            if let Some(data) = &member.data {
                if let Some(role) = data.highest_role_colour() {
                    embed.colour(role.colour);
                }
            }
        }

        user_information(&ctx.author, &user, &mut embed);
        if let Some(ref member) = user.member {
            guild_information(&ctx.author, member, &mut embed);
        }

        ctx.respond(|response| {
            // Handle attempts at stealing data
            if self.gdpr_export.unwrap_or_default() {
                if self.user.is_some() {
                    // TODO: Add privilege esc tally to the person
                    response.content(format!(
                        "Hey <@{}>! <@{}> is being a cunt and trying to steal your data.",
                        user.user_id, ctx.author.user_id
                    ));
                } else {
                    response.ephemeral().attachments = Some(vec![Attachment::from_bytes(
                        format!("gdpr-export-{}.txt", ctx.author.user_id),
                        toml::to_string_pretty(&user).unwrap().as_bytes().to_vec(), // TODO: Handle this unwrap
                        1,
                    )]);
                }
            }
            response.add_embed(embed).components(|c| {
                *c = buttons(ctx.guild_id(), true);
                c
            })
        })
        .await
    }
}
