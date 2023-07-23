use std::fmt::Write;
use std::path::Path;

use async_trait::async_trait;
use git2::{ErrorCode, Repository};
use memory_stats::memory_stats;
use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::Interaction;
use twilight_util::builder::embed::{EmbedFooterBuilder, ImageSource};

use crate::{interactions::InteractionResponse, LuroContext, SlashResponse, builder::LuroResponseV2};

use super::LuroCommand;

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
    async fn run_command(self, interaction: Interaction, ctx: LuroContext, _: MessageSender) -> SlashResponse {
        let response = LuroResponseV2::new("about".to_owned(), interaction.clone()).deferred(&ctx, true).await?;

        // Variables
        let mut embed = self.default_embed(&ctx, interaction.guild_id);
        let mut description = String::new();
        let mut framework_owners_list = String::new();
        let current_user = ctx.twilight_client.current_user().await?.model().await?;
        let current_user_avatar = self.get_currentuser_avatar(&current_user);
        let version = env!("CARGO_PKG_VERSION").to_string();

        let owners = ctx.global_data.read().owners.clone();
        for owner in owners {
            let owner = ctx.twilight_client.user(owner).await?.model().await?;
            write!(framework_owners_list, "{} - <@{}>, ", owner.name, owner.id)?;
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
        if let Some(application_owner) = &ctx.twilight_client.current_user_application().await?.model().await?.owner {
            writeln!(
                description,
                "**Primary Owner:** {} - <@{}>",
                application_owner.name, application_owner.id
            )?;
        };

        if !framework_owners_list.is_empty() {
            writeln!(description, "**Administrators:** {}", framework_owners_list)?;
        }
        writeln!(description, "")?;

        if let Some(memory) = self.memory && memory {
            writeln!(description, "-----")?;

            if let Some(usage) = memory_stats() {
                writeln!(
                    description,
                    "**Physical memory usage:** `{} MB`",
                    usage.physical_mem / 1024 / 1024
                )?;
                writeln!(
                    description,
                    "**Virtual memory usage:** `{} MB`",
                    usage.virtual_mem / 1024 / 1024
                )?;
            };
        }

        if let Some(cache_stats) = self.cache && cache_stats {
            let stats = ctx.twilight_cache.stats();
            writeln!(description, "-----")?;
            writeln!(description, "**Cache Stats**\n")?;
            if stats.guilds() != 0 {
                writeln!(description, "**Guilds:** `{}`", stats.guilds())?;
            }
            if stats.channels() != 0 {
                writeln!(description, "**Channels:** `{}`", stats.channels())?;
            }
            if stats.emojis() != 0 {
                writeln!(description, "**Emojis:** `{}`", stats.emojis())?;
            }
            if stats.members() != 0 {
                writeln!(description, "**Members:** `{}`", stats.members())?;
            }
            if stats.presences() != 0 {
                writeln!(description, "**Presences:** `{}`", stats.presences())?;
            }
            if stats.roles() != 0 {
                writeln!(description, "**Roles:** `{}`", stats.roles())?;
            }
            if stats.unavailable_guilds() != 0 {
                writeln!(
                    description,
                    "**Unavailable Guilds:** `{}`",
                    stats.unavailable_guilds()
                )?;
            }
            if stats.guilds() != 0 {
                writeln!(description, "**Users:** `{}`", stats.users())?;
            }
            if stats.voice_states() != 0 {
                writeln!(description, "**Voice States:** `{}`", stats.voice_states())?;
            }
        }

        embed = embed.description(description);

        Ok(response.embed(embed)?.legacy_response(true))
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
