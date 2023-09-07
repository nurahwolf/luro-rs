use luro_framework::{command::LuroCommandTrait, responses::SimpleResponse, Framework, InteractionCommand, LuroInteraction};
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    channel::message::component::{ButtonStyle, SelectMenuType},
    id::{marker::RoleMarker, Id}
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
    bait_label: Option<String>
}
#[async_trait::async_trait]

impl LuroCommandTrait for Menu {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let interaction_author = interaction.author_id();
        let luro_user = ctx.database.get_user(&interaction_author).await?;

        let mut owner_match = false;

        // We are using global data for this one in case an owner was removed from the application live

        for (id, _) in ctx.database.get_staff().await? {
            if interaction_author == id {
                owner_match = true
            }
        }

        if !owner_match {
            return SimpleResponse::PermissionNotBotStaff().respond(&ctx, &interaction).await;
        }

        // SAFETY: This command can only be used in guilds
        let add_buttons = data.rules.is_some() || data.adult.is_some() || data.bait.is_some();

        let accent_colour = interaction.accent_colour(&ctx).await;
        interaction
            .respond(&ctx, |response| {
                response
                    .embed(|embed| {
                        embed
                            .colour(accent_colour)
                            .author(|author| author.name(luro_user.name()).icon_url(luro_user.avatar()));
                        match data.description {
                            Some(description) => embed.description(description),
                            None => embed.description("Select the roles you want")
                        };
                        if let Some(title) = data.title {
                            embed.title(title);
                        }
                        embed
                    })
                    .components(|components| {
                        components.action_row(|row| {
                            row.component(|component| {
                                component
                                    .select_menu(|menu| menu.custom_id("role-menu").kind(SelectMenuType::Role).max_values(25))
                            })
                        })
                    });

                if add_buttons {
                    response.components(|components| {
                        components.action_row(|row| {
                            if let Some(role) = data.rules {
                                let role = ctx.cache.role(role).unwrap().clone();
                                row.button(|button| {
                                    button.custom_id("rules-button").style(ButtonStyle::Primary);
                                    match data.rules_label {
                                        Some(label) => button.label(label),
                                        None => button.label(role.name.clone())
                                    };
                                    button
                                });
                            }
                            if let Some(role) = data.adult {
                                let role = ctx.cache.role(role).unwrap().clone();
                                row.button(|button| {
                                    button.custom_id("adult-button").style(ButtonStyle::Primary);
                                    match data.adult_label {
                                        Some(label) => button.label(label),
                                        None => button.label(role.name.clone())
                                    };
                                    button
                                });
                            }
                            if let Some(role) = data.bait {
                                let role = ctx.cache.role(role).unwrap().clone();
                                row.button(|button| {
                                    button.custom_id("bait-button").style(ButtonStyle::Danger);
                                    match data.bait_label {
                                        Some(label) => button.label(label),
                                        None => button.label(role.name.clone())
                                    };
                                    button
                                });
                            }
                            row
                        })
                    });
                }
                response
            })
            .await
    }
}
