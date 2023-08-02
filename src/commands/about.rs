use std::fmt::Write;
use std::path::Path;

use async_trait::async_trait;
use git2::{ErrorCode, Repository};
use memory_stats::memory_stats;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::{EmbedFieldBuilder, EmbedFooterBuilder, ImageSource};

use crate::models::LuroSlash;

use crate::traits::luro_command::LuroCommand;
use crate::traits::luro_functions::LuroFunctions;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "about", desc = "Information about me!")]
pub struct AboutCommand {
    /// Show memory stats
    memory: Option<bool>,
    /// Show cache stats,
    cache: Option<bool>
}

#[async_trait]
impl LuroCommand for AboutCommand {
    async fn run_command(self, mut ctx: LuroSlash) -> anyhow::Result<()> {
        ctx.ephemeral().deferred().await?;
        // Variables
        let mut embed = ctx.default_embed().await?;
        let mut description = String::new();
        let mut framework_owners_list = String::new();
        let current_user = ctx.luro.twilight_client.current_user().await?.model().await?;
        let current_user_avatar = ctx.currentuser_get_avatar(&current_user);
        let version = env!("CARGO_PKG_VERSION").to_string();

        for owner in &ctx.luro.global_data.read().owners {
            if framework_owners_list.is_empty() {
                write!(framework_owners_list, "`{}` - <@{}>", owner.name, owner.id)?;
                continue;
            }

            write!(framework_owners_list, ", `{}` - <@{}>", owner.name, owner.id)?;
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

        embed = embed.title(current_user.name);
        embed = embed.thumbnail(ImageSource::url(current_user_avatar)?);
        embed = embed.footer(EmbedFooterBuilder::new("Written in twilight.rs!"));
        // if let Some(git_url) = &ctx.data().config.read().await.git_url {
        //     embed.url(git_url);
        // }
        if let Some(application_owner) = &ctx
            .luro
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
            let stats = ctx.luro.twilight_cache.stats();
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
