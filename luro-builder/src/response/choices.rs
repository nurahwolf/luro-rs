use twilight_model::application::command::CommandOptionChoice;

use super::LuroResponse;

impl LuroResponse {
    /// Set the custom ID
    pub fn choices<I>(&mut self, choices: I) -> &mut Self
    where
        I: Iterator<Item = CommandOptionChoice>
    {
        match &mut self.choices {
            Some(existing_choices) => existing_choices.extend(choices),
            None => self.choices = Some(choices.collect())
        }
        self
    }
}
