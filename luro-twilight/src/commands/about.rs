use std::fmt::Write;
use std::path::Path;

use async_trait::async_trait;
use git2::{ErrorCode, Repository};
use luro_builder::embed::EmbedBuilder;
use luro_framework::{
    command::{ExecuteLuroCommand, LuroCommandTrait},
    CommandInteraction, interactions::InteractionTrait,
};
use luro_model::user::LuroUser;
use memory_stats::memory_stats;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "about", desc = "Information about me!")]
pub struct About {
    /// Show memory stats
    memory: Option<bool>,
    /// Show cache stats,
    cache: Option<bool>,
    /// Show as the user's username instead of ID,
    show_username: Option<bool>,
}

#[async_trait]
impl ExecuteLuroCommand for About {
    async fn interaction_command(&self, ctx: CommandInteraction<()>) -> anyhow::Result<()> {
        let mut description =
            "Hiya! I'm a general purpose Discord bot that can do a good amount of things, complete with a furry twist.\n\n"
                .to_owned();
        let staff = ctx.database.get_staff().await?;
        let current_user = ctx.twilight_client.current_user().await?.model().await?;
        let mut embed = EmbedBuilder::default();
        let slash_author = LuroUser::from(&current_user);

        // Configuration
        embed.colour(ctx.accent_colour().await);
        embed.title(&slash_author.name);
        embed.thumbnail(|thumbnail| thumbnail.url(slash_author.avatar()));
        embed.footer(|footer| footer.text("Written in twilight.rs!"));

        // Build our line processor for calculating padding
        let mut description_builder = vec![];
        description_builder.push(("- Version:", format!("`{}`", env!("CARGO_PKG_VERSION").to_string())));
        if let Ok(repo) = Repository::open(Path::new(env!("CARGO_MANIFEST_DIR")).join("..")) {
            description_builder.push(("- Branch:", format!("`{}`", get_current_branch(&repo))));
            description_builder.push(("- Revision:", format!("`{}`", get_head_revision(&repo))));
        }
        let word_sizes: Vec<(usize, usize)> = description_builder
            .iter()
            .map(|(prefix, suffix)| (prefix.len(), suffix.len()))
            .collect();
        let (prefix_len, suffix_len, _) = padding_calculator(word_sizes);
        for (prefix, suffix) in description_builder {
            writeln!(description, "{prefix:<prefix_len$} {suffix:>suffix_len$}")?;
        }
        embed.description(description);

        if self.cache.unwrap_or_default() {
            let mut cache = String::new();
            let mut description_builder = vec![];
            let stats = ctx.cache.stats();
            writeln!(cache, "```")?;

            if stats.guilds() != 0 {
                description_builder.push(("- Guilds:", stats.guilds().to_string()));
            }
            if stats.channels() != 0 {
                description_builder.push(("- Channels:", stats.channels().to_string()));
            }
            if stats.emojis() != 0 {
                description_builder.push(("- Emojis:", stats.emojis().to_string()));
            }
            if stats.members() != 0 {
                description_builder.push(("- Members:", stats.members().to_string()));
            }
            if stats.presences() != 0 {
                description_builder.push(("- Presences:", stats.presences().to_string()));
            }
            if stats.roles() != 0 {
                description_builder.push(("- Roles:", stats.roles().to_string()));
            }
            if stats.unavailable_guilds() != 0 {
                description_builder.push(("- Unavailable Guilds:", stats.unavailable_guilds().to_string()));
            }
            if stats.guilds() != 0 {
                description_builder.push(("- Users:", stats.users().to_string()));
            }
            if stats.voice_states() != 0 {
                description_builder.push(("- Voice States:", stats.voice_states().to_string()));
            }

            let word_sizes: Vec<(usize, usize)> = description_builder
                .iter()
                .map(|(prefix, suffix)| (prefix.len(), suffix.len()))
                .collect();
            let (prefix_len, suffix_len, _) = padding_calculator(word_sizes);
            for (prefix, suffix) in description_builder {
                writeln!(cache, "{prefix:<prefix_len$} {suffix:>suffix_len$}")?;
            }
            writeln!(cache, "```")?;
            embed.field(|field| field.field("Cache Stats", &cache, false));
        }

        if self.memory.unwrap_or_default() && let Some(usage) = memory_stats() {
            let mut memory =  String::new();
            let mut description_builder = vec![];
            description_builder.push(("- Physical memory usage:", format!("`{} MB`", usage.physical_mem / 1024 / 1024)));
            description_builder.push(("- Virtual memory usage:", format!("`{} MB`", usage.virtual_mem / 1024 / 1024)));
            let word_sizes: Vec<(usize, usize)> = description_builder.iter().map(|(prefix, suffix)| (prefix.len(), suffix.len())).collect();
            let (prefix_len, suffix_len, _) = padding_calculator(word_sizes);
            for (prefix, suffix) in description_builder {
                writeln!(memory, "{prefix:<prefix_len$} {suffix:>suffix_len$}")?;
            }
            embed.field(|field|field.field("Memory Stats", &memory, true));
        }

        if let Some(application_owner) = &ctx.twilight_client.current_user_application().await?.model().await?.owner {
            embed.field(|field| match &self.show_username.unwrap_or_default() {
                true => field.field("My Creator!", &format!("- {}", application_owner.name), true),
                false => field.field("My Creator!", &format!("- <@{}>", application_owner.id), true),
            });
        }

        let mut staff_list = String::new();
        for staff in staff.values() {
            match self.show_username.unwrap_or_default() {
                true => writeln!(staff_list, "- {}", &staff.name)?,
                false => writeln!(staff_list, "- <@{}>", staff.id)?,
            }
        }
        embed.field(|field| field.field("Those with 'Administrator' access!", &staff_list, false));

        ctx.respond(|r| r.add_embed(embed)).await
    }
}

#[async_trait]
impl LuroCommandTrait for About {
    async fn handle_interaction(ctx: CommandInteraction<Self>) -> anyhow::Result<()> {
        let data = Self::new(ctx.data.clone())?;
        let mut description =
            "Hiya! I'm a general purpose Discord bot that can do a good amount of things, complete with a furry twist.\n\n"
                .to_owned();
        let staff = ctx.database.get_staff().await?;
        let current_user = ctx.twilight_client.current_user().await?.model().await?;
        let mut embed = EmbedBuilder::default();
        let slash_author = LuroUser::from(&current_user);

        // Configuration
        embed.colour(ctx.accent_colour().await);
        embed.title(&slash_author.name);
        embed.thumbnail(|thumbnail| thumbnail.url(slash_author.avatar()));
        embed.footer(|footer| footer.text("Written in twilight.rs!"));

        // Build our line processor for calculating padding
        let mut description_builder = vec![];
        description_builder.push(("- Version:", format!("`{}`", env!("CARGO_PKG_VERSION").to_string())));
        if let Ok(repo) = Repository::open(Path::new(env!("CARGO_MANIFEST_DIR")).join("..")) {
            description_builder.push(("- Branch:", format!("`{}`", get_current_branch(&repo))));
            description_builder.push(("- Revision:", format!("`{}`", get_head_revision(&repo))));
        }
        let word_sizes: Vec<(usize, usize)> = description_builder
            .iter()
            .map(|(prefix, suffix)| (prefix.len(), suffix.len()))
            .collect();
        let (prefix_len, suffix_len, _) = padding_calculator(word_sizes);
        for (prefix, suffix) in description_builder {
            writeln!(description, "{prefix:<prefix_len$} {suffix:>suffix_len$}")?;
        }
        embed.description(description);

        if data.cache.unwrap_or_default() {
            let mut cache = String::new();
            let mut description_builder = vec![];
            let stats = ctx.cache.stats();
            writeln!(cache, "```")?;

            if stats.guilds() != 0 {
                description_builder.push(("- Guilds:", stats.guilds().to_string()));
            }
            if stats.channels() != 0 {
                description_builder.push(("- Channels:", stats.channels().to_string()));
            }
            if stats.emojis() != 0 {
                description_builder.push(("- Emojis:", stats.emojis().to_string()));
            }
            if stats.members() != 0 {
                description_builder.push(("- Members:", stats.members().to_string()));
            }
            if stats.presences() != 0 {
                description_builder.push(("- Presences:", stats.presences().to_string()));
            }
            if stats.roles() != 0 {
                description_builder.push(("- Roles:", stats.roles().to_string()));
            }
            if stats.unavailable_guilds() != 0 {
                description_builder.push(("- Unavailable Guilds:", stats.unavailable_guilds().to_string()));
            }
            if stats.guilds() != 0 {
                description_builder.push(("- Users:", stats.users().to_string()));
            }
            if stats.voice_states() != 0 {
                description_builder.push(("- Voice States:", stats.voice_states().to_string()));
            }

            let word_sizes: Vec<(usize, usize)> = description_builder
                .iter()
                .map(|(prefix, suffix)| (prefix.len(), suffix.len()))
                .collect();
            let (prefix_len, suffix_len, _) = padding_calculator(word_sizes);
            for (prefix, suffix) in description_builder {
                writeln!(cache, "{prefix:<prefix_len$} {suffix:>suffix_len$}")?;
            }
            writeln!(cache, "```")?;
            embed.field(|field| field.field("Cache Stats", &cache, false));
        }

        if data.memory.unwrap_or_default() && let Some(usage) = memory_stats() {
            let mut memory =  String::new();
            let mut description_builder = vec![];
            description_builder.push(("- Physical memory usage:", format!("`{} MB`", usage.physical_mem / 1024 / 1024)));
            description_builder.push(("- Virtual memory usage:", format!("`{} MB`", usage.virtual_mem / 1024 / 1024)));
            let word_sizes: Vec<(usize, usize)> = description_builder.iter().map(|(prefix, suffix)| (prefix.len(), suffix.len())).collect();
            let (prefix_len, suffix_len, _) = padding_calculator(word_sizes);
            for (prefix, suffix) in description_builder {
                writeln!(memory, "{prefix:<prefix_len$} {suffix:>suffix_len$}")?;
            }
            embed.field(|field|field.field("Memory Stats", &memory, true));
        }

        if let Some(application_owner) = &ctx.twilight_client.current_user_application().await?.model().await?.owner {
            embed.field(|field| match data.show_username.unwrap_or_default() {
                true => field.field("My Creator!", &format!("- {}", application_owner.name), true),
                false => field.field("My Creator!", &format!("- <@{}>", application_owner.id), true),
            });
        }

        let mut staff_list = String::new();
        for staff in staff.values() {
            match data.show_username.unwrap_or_default() {
                true => writeln!(staff_list, "- {}", &staff.name)?,
                false => writeln!(staff_list, "- <@{}>", staff.id)?,
            }
        }
        embed.field(|field| field.field("Those with 'Administrator' access!", &staff_list, false));

        ctx.respond(|r| r.add_embed(embed)).await
    }
}

/// Retrieves the current git branch in a given git repository.
fn get_current_branch(repo: &Repository) -> String {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => None,
        Err(e) => return format!("An error occured: {e:?}"),
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

/// Work out how many padding characters is needed for a nicely formatted table.
/// This takes a vector containing the word / number lengths in base10, and provices you with the lenth
/// This is broken up by the length of the prefix, suffix and together.
fn padding_calculator(input: Vec<(usize, usize)>) -> (usize, usize, usize) {
    let mut prefix_length = 0;
    let mut suffix_length = 0;

    for (prefix, suffix) in input {
        if prefix > prefix_length {
            prefix_length = prefix
        }

        if suffix > suffix_length {
            suffix_length = suffix
        }
    }

    (prefix_length, suffix_length, prefix_length + suffix_length)
}