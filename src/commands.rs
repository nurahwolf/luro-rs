use twilight_interactions::command::CreateCommand;

use self::dice::DiceCommands;
use self::{
    about::AboutCommand, base64::Base64Commands, boop::BoopCommand, count::CountCommand, heck::HeckCommands,
    hello::HelloCommand, lewd::LewdCommands, moderator::ModeratorCommands, music::MusicCommands, owner::OwnerCommands,
    say::SayCommand, story::StoryCommand, user::UserCommands, uwu::UwUCommand, wordcount::WordcountCommand
};

use anyhow::bail;

use twilight_model::application::interaction::InteractionData;

use crate::models::{Commands, LuroSlash};
use crate::traits::luro_command::LuroCommand;

mod about;
pub mod base64;
pub mod boop;
mod count;
mod dice;
pub mod heck;
mod hello;
mod lewd;
mod moderator;
mod music;
mod owner;
mod say;
mod story;
mod user;
mod uwu;
mod wordcount;
// pub mod fursona;

impl Commands {
    /// Return a default set of commands to register
    pub fn default_commands() -> Self {
        // Create the hashmaps
        let mut init = Self {
            guild_commands: Default::default(),
            global_commands: Default::default()
        };

        // Add some default commands
        init.global_commands.insert("about", AboutCommand::create_command().into());
        init.global_commands.insert("hello", HelloCommand::create_command().into());
        init.global_commands.insert("count", CountCommand::create_command().into());
        init.global_commands.insert("say", SayCommand::create_command().into());
        init.global_commands.insert("mod", ModeratorCommands::create_command().into());
        init.global_commands.insert("music", MusicCommands::create_command().into());
        init.global_commands.insert("boop", BoopCommand::create_command().into());
        init.global_commands.insert("heck", HeckCommands::create_command().into());
        init.global_commands.insert("owner", OwnerCommands::create_command().into());
        init.global_commands.insert("about", AboutCommand::create_command().into());
        init.global_commands.insert("user", UserCommands::create_command().into());
        init.global_commands.insert("lewd", LewdCommands::create_command().into());
        init.global_commands.insert("base64", Base64Commands::create_command().into());
        init.global_commands.insert("story", StoryCommand::create_command().into());
        init.global_commands.insert("uwu", UwUCommand::create_command().into());
        init.global_commands.insert("roll", DiceCommands::create_command().into());

        init.global_commands
            .insert("wordcount", WordcountCommand::create_command().into());

        // Return our initialised commands
        init
    }
}

impl LuroSlash {
    /// Handle incoming command interaction.
    pub async fn handle_command(self) -> anyhow::Result<()> {
        let data = match self.interaction.data.clone() {
            Some(InteractionData::ApplicationCommand(data)) => *data,
            _ => bail!("expected application command data")
        };

        match data.name.as_str() {
            "about" => AboutCommand::new(data).await?.run_command(self).await,
            "say" => SayCommand::new(data).await?.run_command(self).await,
            "user" => UserCommands::new(data).await?.run_command(self).await,
            "hello" => HelloCommand::new(data).await?.run_command(self).await,
            "count" => CountCommand::new(data).await?.run_command(self).await,
            "mod" => ModeratorCommands::new(data).await?.run_commands(self).await,
            "music" => MusicCommands::new(data).await?.run_commands(self).await,
            "boop" => BoopCommand::new(data).await?.run_command(self).await,
            "owner" => OwnerCommands::new(data).await?.run_commands(self).await,
            "heck" => HeckCommands::new(data).await?.run_commands(self).await,
            "lewd" => LewdCommands::new(data).await?.run_commands(self).await,
            "base64" => Base64Commands::new(data).await?.run_commands(self).await,
            "story" => StoryCommand::new(data).await?.run_command(self).await,
            "uwu" => UwUCommand::new(data).await?.run_command(self).await,
            "wordcount" => WordcountCommand::new(data).await?.run_command(self).await,
            "roll" => DiceCommands::new(data).await?.run_commands(self).await,
            _ => self.unknown_command_response().await
        }
    }
}
