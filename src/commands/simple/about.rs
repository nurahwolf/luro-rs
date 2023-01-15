use crate::{functions::guild_accent_colour::guild_accent_colour, Context, Error};
use git2::{ErrorCode, Repository};
use memory_stats::memory_stats;
use poise::serenity_prelude::{CacheHttp, CreateEmbed};
use std::fmt::Write;

/// Retrieves the current git branch in a given git repository.
pub fn get_current_branch(repo: &Repository) -> String {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => None,
        Err(e) => return format!("An error occured: {e:?}")
    };

    let head = head.as_ref().and_then(|h| h.shorthand());
    head.unwrap().to_string()
}

/// Retrieves the latest HEAD revision for the given git repository.
pub fn get_head_revision(repo: &Repository) -> String {
    let revspec = repo.revparse("HEAD").unwrap();
    let revision = revspec.from().unwrap();
    revision.short_id().unwrap().as_str().unwrap().to_string()
}

/// Information about the bot!
#[poise::command(prefix_command, slash_command, category = "General")]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    // Variables
    let mut embed = CreateEmbed::default();
    let accent_colour = ctx.data().config.read().await.accent_colour;
    let current_user = ctx.serenity_context().cache.current_user();
    let repo = Repository::open(env!("CARGO_MANIFEST_DIR"))?;
    let version = env!("CARGO_PKG_VERSION").to_string();
    let branch = get_current_branch(&repo);
    let revision = get_head_revision(&repo);
    let application_info = ctx.http().get_current_application_info().await?;
    let framework_owners = &ctx.framework().options.owners;
    let mut framework_owners_list = String::new();
    for owner in framework_owners {
        match ctx.http().get_user(owner.0).await {
            Ok(user) => {
                write!(framework_owners_list, "{} ({}), ", user, user.tag())?;
            }
            Err(_) => todo!()
        }
    }

    // Embed
    embed.title(&current_user.name);
    embed.color(guild_accent_colour(accent_colour, ctx.guild()));
    embed.thumbnail(&current_user.avatar_url().unwrap_or_default());
    embed.footer(|footer| footer.text("Written with Rust & Poise (Serenity)"));
    if let Some(git_url) = &ctx.data().config.read().await.git_url {
        embed.url(git_url);
    }

    let mut fields = vec![
        ("Version", version, true),
        ("Branch", branch, true),
        ("Revision", format!("`{revision}`"), true),
        (
            "Main Bot Owner",
            format!("{} ({})", application_info.owner, application_info.owner.tag()),
            true
        ),
    ];

    if let Some(usage) = memory_stats() {
        fields.push((
            "Physical memory usage",
            format!("{}MB", usage.physical_mem / 1024 / 1024),
            true
        ));
        fields.push(("Virtual memory usage", format!("{}MB", usage.virtual_mem / 1024 / 1024), true));
    };

    if let Some(cache) = ctx.cache() {
        fields.push(("Shards", cache.shard_count().to_string(), true));
        fields.push(("Guilds", cache.guilds().len().to_string(), true));
        fields.push(("Channels", cache.guild_channel_count().to_string(), true));
        fields.push(("Users", cache.user_count().to_string(), true));
    }
    fields.push(("Users with Owner perms", framework_owners_list, false));
    embed.fields(fields);

    ctx.send(|builder| {
        builder.embed(|e| {
            *e = embed;
            e
        })
    })
    .await?;

    Ok(())
}
