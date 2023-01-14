use poise::serenity_prelude::Interaction;

use crate::Error;

/// A Serenity listener for the [poise::Event::InteractionCreate] type
pub async fn interaction_create(interaction: &Interaction) -> Result<(), Error> {
    match interaction.clone().application_command() {
        Some(interaction_command) => {
            println!("Event Listener: Data - {}", interaction_command.data.name)
        }
        None => println!("Event Listener: {}", interaction.id().0)
    };
    Ok(())
}
