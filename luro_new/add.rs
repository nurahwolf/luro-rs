use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::Interaction,
    channel::message::{
        component::{ActionRow, SelectMenu, SelectMenuOption},
        Component, ReactionType,
    },
    id::Id,
};

use crate::interactions::InteractionResponse;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "add", desc = "Add a heck", dm_permission = true)]
pub struct HeckAddCommand {}

impl HeckAddCommand {
    pub async fn run(&self, interaction: &Interaction) -> anyhow::Result<InteractionResponse> {
        tracing::debug!(
            "heck user command in channel {} by {}",
            interaction.channel.clone().unwrap().name.unwrap(),
            interaction.user.clone().unwrap().name
        );

        let action_row = Component::ActionRow(ActionRow {
            components: vec![Component::SelectMenu(SelectMenu {
                custom_id: "class_select_1".to_owned(),
                disabled: false,
                max_values: Some(1),
                min_values: Some(1),
                options: Vec::from([
                    SelectMenuOption {
                        default: false,
                        emoji: Some(ReactionType::Custom {
                            animated: false,
                            id: Id::new(625891304148303894),
                            name: Some("rogue".to_owned()),
                        }),
                        description: Some("Sneak n stab".to_owned()),
                        label: "Rogue".to_owned(),
                        value: "rogue".to_owned(),
                    },
                    SelectMenuOption {
                        default: false,
                        emoji: Some(ReactionType::Custom {
                            animated: false,
                            id: Id::new(625891304081063986),
                            name: Some("mage".to_owned()),
                        }),
                        description: Some("Turn 'em into a sheep".to_owned()),
                        label: "Mage".to_owned(),
                        value: "mage".to_owned(),
                    },
                    SelectMenuOption {
                        default: false,
                        emoji: Some(ReactionType::Custom {
                            animated: false,
                            id: Id::new(625891303795982337),
                            name: Some("priest".to_owned()),
                        }),
                        description: Some("You get heals when I'm done doing damage".to_owned()),
                        label: "Priest".to_owned(),
                        value: "priest".to_owned(),
                    },
                ]),
                placeholder: Some("Choose a class".to_owned()),
            })],
        });

        Ok(InteractionResponse::Text {
            content: "Work in progress".to_string(),
            components: Some(vec![action_row]),
            ephemeral: false,
        })
    }
}
