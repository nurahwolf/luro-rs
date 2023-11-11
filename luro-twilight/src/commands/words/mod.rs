use luro_framework::{CommandInteraction, CreateLuroCommand, LuroCommand};
use tabled::{builder::Builder, settings::Style, Table};
use twilight_interactions::command::{CommandModel, CreateCommand};

mod global;
mod guild;
mod personal;

#[derive(CommandModel, CreateCommand)]
#[command(name = "words", desc = "Stats for words said. These are global metrics (by default)!")]
pub enum Words {
    #[command(name = "global")]
    Global(global::Global),
    #[command(name = "guild")]
    Guild(guild::Guild),
    #[command(name = "personal")]
    Personal(personal::Personal),
}

impl CreateLuroCommand for Words {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        match self {
            Self::Global(command) => command.interaction_command(ctx).await,
            Self::Guild(command) => command.interaction_command(ctx).await,
            Self::Personal(command) => command.interaction_command(ctx).await,
        }
    }
}

#[derive(twilight_interactions::command::CommandOption, twilight_interactions::command::CreateOption)]
pub enum TableStyle {
    #[option(name = "ASCII - An ASCII table. What more could you ask for?", value = "ascii")]
    Ascii,
    #[option(name = "ASCII Rounded - Rounded table, using ASCII characters instead", value = "ascii_rounded")]
    AsciiRounded,
    #[option(name = "Blank - Uses empty spaces to represent the table", value = "blank")]
    Blank,
    #[option(name = "Dots - Dots make up the table", value = "dots")]
    Dots,
    #[option(name = "Markdown - A markdown table", value = "markdown")]
    Markdown,
    #[option(name = "Modern - Super slick using UTF-8", value = "modern")]
    Modern,
    #[option(name = "PSQL (Default) - Postgress looking table, super simple and minimal", value = "psql")]
    Psql,
    #[option(name = "Rounded - Like modern, but well... Rounded?", value = "rounded")]
    Rounded,
    #[option(name = "Sharp - Modern, without the horizontal line", value = "sharp")]
    Sharp,
}

pub fn table_style(table: Builder, style: Option<&TableStyle>) -> Table {
    let mut table = table.build();
    match style {
        Some(style) => match style {
            TableStyle::Ascii => table.with(Style::ascii()),
            TableStyle::AsciiRounded => table.with(Style::ascii_rounded()),
            TableStyle::Blank => table.with(Style::blank()),
            TableStyle::Dots => table.with(Style::dots()),
            TableStyle::Markdown => table.with(Style::markdown()),
            TableStyle::Modern => table.with(Style::modern()),
            TableStyle::Psql => table.with(Style::psql()),
            TableStyle::Rounded => table.with(Style::rounded()),
            TableStyle::Sharp => table.with(Style::sharp()),
        },
        None => table.with(Style::psql()),
    };

    table
}
