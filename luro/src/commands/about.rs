use std::convert::TryInto;
use std::fmt::Write;
use std::path::Path;


use git2::{ErrorCode, Repository};
use luro_model::constants::PRIMARY_BOT_OWNER;
use memory_stats::memory_stats;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::{EmbedFieldBuilder, EmbedFooterBuilder};

use crate::models::SlashUser;

use crate::slash::Slash;
use crate::traits::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "about", desc = "Information about me!")]
pub struct AboutCommand {
    /// Show memory stats
    memory: Option<bool>,
    /// Show cache stats,
    cache: Option<bool>
}


impl LuroCommand for AboutCommand {
    async fn run_command(self, mut ctx: Slash) -> anyhow::Result<()> {
        ctx.ephemeral().deferred().await?;
        // Variables
        let mut embed = ctx.default_embed().await?;
        let mut description = String::new();
        let mut framework_owners_list = String::new();
        let current_user = ctx.framework.twilight_client.current_user().await?.model().await?;
        let slash_author = SlashUser::from(current_user);
        let version = env!("CARGO_PKG_VERSION").to_string();

        let staff = ctx.framework.database.get_staff().await?;
        for staff in staff.iter() {
            if framework_owners_list.is_empty() {
                write!(
                    framework_owners_list,
                    "`{}` - <@{}>",
                    staff.name.clone().unwrap_or("unknown".to_owned()),
                    staff.id.unwrap_or(PRIMARY_BOT_OWNER)
                )?;
                continue;
            }

            write!(
                framework_owners_list,
                ", `{}` - <@{}>",
                staff.name.clone().unwrap_or("unknown".to_owned()),
                staff.id.unwrap_or(PRIMARY_BOT_OWNER)
            )?;
        }
        writeln!(
            description,
            "Hiya! I'm a general purpose Discord bot that can do a good amount of things, complete with a furry twist."
        )?;
        writeln!(description, "**\nVersion:** `{}`", version)?;

        // If we are git
        if let Ok(repo) = Repository::open(Path::new(env!("CARGO_MANIFEST_DIR")).join("..")) {
            let branch = get_current_branch(&repo);
            let revision = get_head_revision(&repo);

            writeln!(description, "**Branch:** `{}`", branch)?;
            writeln!(description, "**Revision:** `{}`", revision)?;
        }

        embed = embed.title(&slash_author.name);
        embed = embed.thumbnail(slash_author.clone().try_into()?);
        embed = embed.footer(EmbedFooterBuilder::new("Written in twilight.rs!"));
        // if let Some(git_url) = &ctx.data().config.read().await.git_url {
        //     embed.url(git_url);
        // }
        if let Some(application_owner) = &ctx
            .framework
            .twilight_client
            .current_user_application()
            .await?
            .model()
            .await?
            .owner
        {
            writeln!(
                description,
                "**Primary Owner:** `{}` - <@{}>",
                application_owner.name, application_owner.id
            )?;
        };

        if !framework_owners_list.is_empty() {
            writeln!(description, "**Administrators:** {}", framework_owners_list)?;
        }
        writeln!(description, "")?;

        if let Some(memory) = self.memory && memory {
            let mut memory_description = String::new();
            if let Some(usage) = memory_stats() {
                writeln!(
                    memory_description,
                    "**Physical memory usage:** `{} MB`",
                    usage.physical_mem / 1024 / 1024
                )?;
                writeln!(
                    memory_description,
                    "**Virtual memory usage:** `{} MB`",
                    usage.virtual_mem / 1024 / 1024
                )?;
            };
            embed = embed.field(EmbedFieldBuilder::new("Memory Stats", memory_description).inline())
        }

        if let Some(cache_stats) = self.cache && cache_stats {
            let mut cache_stats = String::new();
            let stats = ctx.framework.twilight_cache.stats();
            if stats.guilds() != 0 {
                writeln!(cache_stats, "**Guilds:** `{}`", stats.guilds())?;
            }
            if stats.channels() != 0 {
                writeln!(cache_stats, "**Channels:** `{}`", stats.channels())?;
            }
            if stats.emojis() != 0 {
                writeln!(cache_stats, "**Emojis:** `{}`", stats.emojis())?;
            }
            if stats.members() != 0 {
                writeln!(cache_stats, "**Members:** `{}`", stats.members())?;
            }
            if stats.presences() != 0 {
                writeln!(cache_stats, "**Presences:** `{}`", stats.presences())?;
            }
            if stats.roles() != 0 {
                writeln!(cache_stats, "**Roles:** `{}`", stats.roles())?;
            }
            if stats.unavailable_guilds() != 0 {
                writeln!(
                    cache_stats,
                    "**Unavailable Guilds:** `{}`",
                    stats.unavailable_guilds()
                )?;
            }
            if stats.guilds() != 0 {
                writeln!(cache_stats, "**Users:** `{}`", stats.users())?;
            }
            if stats.voice_states() != 0 {
                writeln!(cache_stats, "**Voice States:** `{}`", stats.voice_states())?;
            }
            embed = embed.field(EmbedFieldBuilder::new("Cache Stats", cache_stats).inline())
        }

        embed = embed.description(description);

        ctx.embed(embed.build())?.respond().await
    }
}

/// Retrieves the current git branch in a given git repository.
fn get_current_branch(repo: &Repository) -> String {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => None,
        Err(e) => return format!("An error occured: {e:?}")
    };

    let head = head.as_ref().and_then(|h| h.shorthand());
    head.unwrap().to_string()
}

/// Retrieves the latest HEAD revision for the given git repository.
fn get_head_revision(repo: &Repository) -> String {
    let revspec = repo.revparse("HEAD").unwrap();
    let revision = revspec.from().unwrap();
    revision.short_id().unwrap().as_str().unwrap().to_string()
}
