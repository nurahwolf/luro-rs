use luro_framework::Luro;
use luro_model::builders::components::action_row::ActionRowBuilder;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    channel::message::component::{ButtonStyle, SelectMenuType},
    id::{marker::RoleMarker, Id},
};

#[derive(CommandModel, CreateCommand)]
#[command(name = "menu", desc = "Show a role menu, for easily selecting roles")]
pub struct Menu {
    /// Customise the embed description. Leave blank for the default
    description: Option<String>,
    /// Customise the embed title. Leave blank for none
    title: Option<String>,
    /// Role to give to those who agree to the rules (e.g. Minor)
    pub rules: Option<Id<RoleMarker>>,
    /// The button's label. Defaults to the role name
    rules_label: Option<String>,
    /// Role to give to those who agree to the rules AND are over 18
    pub adult: Option<Id<RoleMarker>>,
    /// The button's label. Defaults to the role name
    adult_label: Option<String>,
    /// Role to give to those that clicked the funny button
    pub bait: Option<Id<RoleMarker>>,
    /// The button's label. Defaults to the role name
    bait_label: Option<String>,
    /// Set a banner to show below the embed
    banner: Option<String>
}

impl luro_framework::LuroCommand for Menu {
    async fn interaction_command(self, ctx: luro_framework::CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        let guild_id = ctx.guild_id().unwrap(); // SAFETY: Safe to unwrap as this can only be run in a guild
        let mut owner_match = false;

        for staff in ctx.database.user_fetch_staff().await? {
            if ctx.author.user_id == staff.user_id {
                owner_match = true
            }
        }

        if !owner_match {
            return ctx
                .simple_response(luro_model::response::SimpleResponse::PermissionNotBotStaff)
                .await;
        }

        // SAFETY: This command can only be used in guilds
        let add_buttons = self.rules.is_some() || self.adult.is_some() || self.bait.is_some();
        let mut action_row = ActionRowBuilder::default();
        if let Some(role) = self.rules {
            let role = ctx.database.role_fetch(guild_id, role).await?;
            action_row.button(|button| {
                button.custom_id("roles-button-rules").style(ButtonStyle::Primary);
                match self.rules_label {
                    Some(label) => button.label(label),
                    None => button.label(role.name.clone()),
                };
                button
            });
        }
        if let Some(role) = self.adult {
            let role = ctx.database.role_fetch(guild_id, role).await?;
            action_row.button(|button| {
                button.custom_id("roles-button-adult").style(ButtonStyle::Primary);
                match self.adult_label {
                    Some(label) => button.label(label),
                    None => button.label(role.name.clone()),
                };
                button
            });
        }
        if let Some(role) = self.bait {
            let role = ctx.database.role_fetch(guild_id, role).await?;
            action_row.button(|button| {
                button.custom_id("roles-button-bait").style(ButtonStyle::Danger);
                match self.bait_label {
                    Some(label) => button.label(label),
                    None => button.label(role.name.clone()),
                };
                button
            });
        }

        ctx.respond(|response| {
            response
                .embed(|embed| {
                    if let Some(banner) = self.banner {
                        embed.image(|i|i.url(banner));
                    }
                    match self.description {
                        Some(description) => embed.description(description),
                        None => embed.description("Select the roles you want"),
                    };
                    if let Some(title) = self.title {
                        embed.title(title);
                    }
                    embed.colour(ctx.accent_colour())
                })
                .components(|components| {
                    components.action_row(|row| {
                        row.component(|component| {
                            component.select_menu(|menu| menu.custom_id("role-menu").kind(SelectMenuType::Role).max_values(25))
                        })
                    })
                });

            if add_buttons {
                response.components(|components| {
                    components.action_row(|a_r| {
                        *a_r = action_row;
                        a_r
                    })
                });
            }
            response
        })
        .await
    }
}
