use super::PrefixCommand;

impl PrefixCommand {
    pub fn name(&self) -> &str {
        self.names[0]
    }
}
