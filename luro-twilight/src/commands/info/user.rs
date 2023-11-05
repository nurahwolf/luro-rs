use luro_framework::{CommandInteraction, Luro, LuroCommand};
use rand::{seq::SliceRandom, thread_rng};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{http::attachment::Attachment, id::Id};

use super::{buttons, timestamps, luro_information, user_information, guild_information};

const REMARK: [&str; 3] = ["Hey <user>!", "Great to see ya, <user>!", "Whoa, it's <user>!"];

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "user", desc = "Information about a user")]
pub struct InfoUser {
    /// Set to true to show some basic information, with buttons to fetch more.
    simple: bool,
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
            user.sync(&ctx.database).await;
        }

        embed
            .author(|author| {
                author
                    .name(format!("Infomation on {} | {}", user.name, user.user_id))
                    .icon_url(user.avatar_url())
            })
            .description(description.replace("<user>", &format!("<@{}>", user.user_id)))
            .footer(|f| {
                f.text(match user.instance {
                    luro_database::LuroUserType::User => "Twilight User - Data fetched using the Discord API",
                    luro_database::LuroUserType::Member => "Twilight Member - Data fetched using the Discord API, including guild data",
                    luro_database::LuroUserType::DbUser => {
                        "Luro User - Data fetched from my database only, with includes your custom stuff!"
                    }
                    luro_database::LuroUserType::DbMember => "Luro Member - Data fetched from my database, including guild information!",
                    luro_database::LuroUserType::DbMemberNoRoles => {
                        "Luro Member without roles - User and member information fetched from my database, but no roles were present"
                    }
                })
            });

        if !self.hide_avatar.unwrap_or_default() {
            embed.thumbnail(|thumbnail| thumbnail.url(user.avatar_url()));
        }

        if let Some(ref banner) = user.banner_url() {
            embed.image(|i| i.url(banner));
        }

        if !self.simple {
            timestamps(&ctx.author, &user, &mut embed);
            luro_information(&ctx.author, &user, ctx.database.clone(), &mut embed).await;
            user_information(&ctx.author, &user, &mut embed);
            if let Some(ref member) = user.member {
                guild_information(&ctx.author, member, &mut embed);
            }

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
                *c = buttons(ctx.guild_id());
                c
            })
        })
        .await
    }
}
