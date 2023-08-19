use std::collections::HashMap;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::application::command::Command;

pub fn luro_command<C: LuroCommandTrait + 'static>(command: C) -> Box<dyn LuroCommandTrait> {
    Box::new(command)
}

pub fn test<'a>() -> HashMap<&'a str, Box<dyn LuroCommandTrait>> {
    let mut map = HashMap::new();
    map.insert("owo", luro_command(Test {}));
    map.insert("uwu", luro_command(TestAgain {}));
    map.insert("uwu", luro_command(TestBetter {}));
    map.insert("nice", luro_command(SayCommand { message: Default::default(), user: None }));
    map
}

pub fn foo() {
    let data = test();
    match data.get("uwu") {
        Some(command) => command,
        None => panic!("Command not found!"),
    };
}

pub fn test_struct<'a>() -> HashMap<&'a str, LuroCommand<'a>> {
    let mut map = HashMap::new();
    map.insert("owo", LuroCommand::from(SayCommand { message: Default::default(), user: None }));
    map
}

pub fn bar() {
    let data = test_struct();
    match data.get("uwu") {
        Some(command) => command,
        None => panic!("Command not found!"),
    };
}

/// A simple wrapper around an interaction command type. This will eventually be replaced in house.
pub struct LuroCommand<'a> {
    /// The name of the command
    pub name: &'a str,
    /// The command as passed to Discord
    pub command: Command,
    /// The data for the command itself
    // pub data: Box<dyn LuroCommandTrait>,
    /// The IDs that this command responds to
    pub ids: Vec<&'a str>
}

impl<'a, C: LuroCommandDerive + 'static> From<C> for LuroCommand<'a> {
    fn from(_value: C) -> Self {
        Self {
            name: C::NAME,
            command: C::create_command().into(),
            // data: luro_command(value),
            ids: vec!["todo"]
        }
    }
}

/// Our main command trait, enforcing a few requirements. This one is Sized
pub trait LuroCommandTrait {}

// impl<'a, C: LuroCommandDerive + LuroCommandTrait + 'static> From<C> for LuroCommand<'a> {
//     fn from(value: C) -> Self {
//         Self {
//             name: C::NAME,
//             command: C::create_command().into(),
//             data: luro_command(value),
//             ids: vec!["todo"]
//         }
//     }
// }

// impl<'a, C: LuroCommandTrait> From<C> for  Box<dyn LuroCommandTrait> {
//     fn from(value: C) -> Self {
//         Box::new(value)
//     }
// }

/// Similar to the above, but sized. This is to allow for nesting
pub trait LuroCommandDerive: CommandModel + CreateCommand {
    async fn run_command(self) -> anyhow::Result<()> {
        Ok(())
    }
}

pub struct Test {}

impl LuroCommandTrait for Test {}

pub struct TestBetter {}

impl LuroCommandTrait for TestBetter {}

pub struct TestAgain {}

impl LuroCommandTrait for TestAgain {}

#[derive(CommandModel, CreateCommand)]
#[command(name = "say", desc = "Make me say garbage!")]
pub struct SayCommand {
    /// The message to send.
    message: String,
    /// The user to send the message to.
    user: Option<ResolvedUser>
}

impl LuroCommandTrait for SayCommand {
    
}

impl LuroCommandDerive for SayCommand {
    async fn run_command(self) -> anyhow::Result<()> {
        println!("It works!");
        Ok(())
    }
}
