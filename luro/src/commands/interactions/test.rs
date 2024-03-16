use luro_model::command::{SlashCommand, SlashContext, SlashResult};

pub fn test_command() -> SlashCommand {
    SlashCommand {
        command: Box::new(move || Box::pin(test())),
        // subcommand: vec![],
        name: "test-command".to_owned(),
        description: "a test command!".to_owned(),
        long_description: None,
        nsfw: false,
        // checks: (),
    }
}

// #[luro_derive::slash_command]
pub async fn test_command_v2(_ctx: SlashContext) -> SlashResult {
    Ok(())
}

async fn test() -> SlashResult {
    Ok(())
}
