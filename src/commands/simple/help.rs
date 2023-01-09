use crate::{functions::guild_accent_colour::guild_accent_colour, Context, Data, Error};
use poise::{
    serenity_prelude::{CacheHttp, CreateEmbed},
    ContextMenuCommandAction, PartialContext
};
use std::fmt::Write as _;

/// Optional configuration for how the help message from [`help()`] looks
pub struct HelpConfiguration<'a> {
    /// Extra text displayed at the bottom of your message. Can be used for help and tips specific
    /// to your bot
    pub extra_text_at_bottom: &'a str,
    /// Whether to make the response ephemeral if possible. Can be nice to reduce clutter
    pub ephemeral: bool,
    /// Whether to list context menu commands as well
    pub show_context_menu_commands: bool,
    /// Hide the bot avatar, useful for giving the embed more space
    pub hide_avatar: bool
}

impl Default for HelpConfiguration<'_> {
    fn default() -> Self {
        Self {
            extra_text_at_bottom: "",
            ephemeral: true,
            show_context_menu_commands: false,
            hide_avatar: false
        }
    }
}

/// Depending on indexmap seems overkill, so this will do instead
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrderedMap<K, V>(pub Vec<(K, V)>);

impl<K, V> Default for OrderedMap<K, V> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K: Eq, V> OrderedMap<K, V> {
    /// Creates a new [`OrderedMap`]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Finds a value in the map by the given key, or inserts it if it doesn't exist
    pub fn get_or_insert_with(&mut self, k: K, v: impl FnOnce() -> V) -> &mut V {
        match self.0.iter().position(|entry| entry.0 == k) {
            Some(i) => &mut self.0[i].1,
            None => {
                self.0.push((k, v()));
                &mut self.0.last_mut().expect("we just inserted").1
            }
        }
    }
}

impl<K, V> IntoIterator for OrderedMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<(K, V)>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Code for printing help of a specific command (e.g. `~help my_command`)
async fn help_single_command(ctx: Context<'_>, command_name: &str, config: HelpConfiguration<'_>) -> Result<(), Error> {
    let command = ctx.framework().options().commands.iter().find(|command| {
        if command.name.eq_ignore_ascii_case(command_name) {
            return true;
        }
        if let Some(context_menu_name) = command.context_menu_name {
            if context_menu_name.eq_ignore_ascii_case(command_name) {
                return true;
            }
        }

        false
    });

    let reply = if let Some(command) = command {
        match command.help_text {
            Some(f) => f(),
            None => command.description.as_deref().unwrap_or("No help available").to_owned()
        }
    } else {
        format!("No such command `{command_name}`")
    };

    let mut embed = CreateEmbed::default();
    embed.title(command_name);
    embed.description(reply);
    embed.colour(guild_accent_colour(ctx.data().config.read().await.accent_colour, ctx.guild()));
    if let Ok(bot_user) = ctx.http().get_user(ctx.framework().bot_id.0).await {
        embed.author(|author| author.name(&bot_user.name).icon_url(&bot_user.avatar_url().unwrap_or_default()));
        if !config.hide_avatar {
            embed.thumbnail(&bot_user.avatar_url().unwrap_or_default());
        };
    }

    ctx.send(|builder| {
        builder
            .embed(|f| {
                *f = embed;
                f
            })
            .ephemeral(config.ephemeral)
    })
    .await?;

    Ok(())
}

/// Code for printing an overview of all commands (e.g. `~help`)
async fn help_all_commands(ctx: Context<'_>, config: HelpConfiguration<'_>) -> Result<(), Error> {
    let mut categories = OrderedMap::<Option<&str>, Vec<&poise::Command<Data, Error>>>::new();
    for cmd in &ctx.framework().options().commands {
        categories.get_or_insert_with(cmd.category, Vec::new).push(cmd);
    }

    let mut embed = CreateEmbed::default();
    embed.title("Help Menu");
    embed.colour(guild_accent_colour(ctx.data().config.read().await.accent_colour, ctx.guild()));
    embed.footer(|footer| footer.text(config.extra_text_at_bottom));
    if let Ok(bot_user) = ctx.http().get_user(ctx.framework().bot_id.0).await {
        embed.author(|author| author.name(&bot_user.name).icon_url(&bot_user.avatar_url().unwrap_or_default()));
        if !config.hide_avatar {
            embed.thumbnail(&bot_user.avatar_url().unwrap_or_default());
        };
    }

    for (category_name, commands) in categories {
        let mut commands_string = String::new();

        for command in commands {
            if command.hide_in_help {
                continue;
            }

            let prefix = if command.slash_action.is_some() {
                String::from("/")
            } else if command.prefix_action.is_some() {
                let options = &ctx.framework().options().prefix_options;

                match &options.prefix {
                    Some(fixed_prefix) => fixed_prefix.clone(),
                    None => match options.dynamic_prefix {
                        Some(dynamic_prefix_callback) => {
                            match dynamic_prefix_callback(PartialContext::from(ctx)).await {
                                Ok(Some(dynamic_prefix)) => dynamic_prefix,
                                // `String::new()` defaults to "" which is what we want
                                Err(_) | Ok(None) => String::new()
                            }
                        }
                        None => String::new()
                    }
                }
            } else {
                // This is not a prefix or slash command, i.e. probably a context menu only command
                // which we will only show later
                continue;
            };

            let total_command_name_length = prefix.chars().count() + command.name.chars().count();
            let padding = 12_usize.saturating_sub(total_command_name_length) + 1;
            writeln!(
                commands_string,
                "{}{}{}{}",
                prefix,
                command.name,
                " ".repeat(padding),
                command.description.as_deref().unwrap_or("")
            )?;
        }
        if let Some(category_name) = category_name {
            let mut commands_string_code_block = "```\n".to_string();
            commands_string_code_block.push_str(commands_string.as_str());
            commands_string_code_block.push_str("\n```");
            embed.field(category_name, commands_string_code_block, false);
        }
    }

    // Show context menu commands
    if config.show_context_menu_commands {
        let mut context_menu_commands = String::new();

        for command in &ctx.framework().options().commands {
            let kind = match command.context_menu_action {
                Some(ContextMenuCommandAction::User(_)) => "user",
                Some(ContextMenuCommandAction::Message(_)) => "message",
                None => continue
            };
            let name = command.context_menu_name.unwrap_or(&command.name);
            writeln!(context_menu_commands, "  {name} (on {kind})")?;
        }

        if !context_menu_commands.is_empty() {
            embed.field("Context menu commands", context_menu_commands, false);
        }
    }

    // Send the embed
    ctx.send(|builder| {
        builder
            .embed(|f| {
                *f = embed;
                f
            })
            .ephemeral(config.ephemeral)
    })
    .await?;

    Ok(())
}

/// Help Command
#[poise::command(prefix_command, track_edits, slash_command, ephemeral, category = "General")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Set to false if you want to see this in the server!"]
    #[flag]
    ephemeral: bool,
    #[description = "Set to true if you want to hide the bot avatar"]
    #[flag]
    hide_avatar: bool,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>
) -> Result<(), Error> {
    let config = HelpConfiguration {
        extra_text_at_bottom: "Contact Nurah#5103 with any questions.",
        show_context_menu_commands: true,
        ephemeral,
        hide_avatar
    };

    match command {
        Some(cmd) => help_single_command(ctx, cmd.as_str(), config).await?,
        None => help_all_commands(ctx, config).await?
    }

    Ok(())
}
