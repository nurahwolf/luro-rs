use std::collections::HashMap;

use twilight_model::application::command::Command;

#[derive(Default)]
pub struct Guild {
    /// Commands registered to a guild
    pub commands: HashMap<&'static str, Command>,
}
