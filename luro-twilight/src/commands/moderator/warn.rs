use async_trait::async_trait;
use luro_framework::command::LuroCommandTrait;
use luro_framework::{Framework, InteractionCommand, LuroInteraction};
use luro_model::database_driver::LuroDatabaseDriver;
use twilight_model::http::interaction::InteractionResponseType;

use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

use twilight_model::channel::message::component::{ActionRow, TextInput, TextInputStyle};
use twilight_model::channel::message::Component;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "warn",
    desc = "Get the warnings of a user, or set a new warning",
    default_permissions = "Self::default_permissions"
)]
pub struct Warn {
    /// Want to make a new warning?
    new: bool,
    /// The user to warn / get warnings for.
    user: ResolvedUser,
}
#[async_trait]
impl LuroCommandTrait for Warn {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let punished_user = ctx.database.get_user(&data.user.resolved.id).await?;

        if !data.new {
            if punished_user.warnings.is_empty() {
                return interaction.respond(&ctx, |r| r.content("No warnings for that user!")).await;
            }

            let mut warnings_formatted = String::new();
            for (warning, user_id) in &punished_user.warnings {
                writeln!(warnings_formatted, "Warning by <@{user_id}>```{warning}```")?
            }

            let accent_colour = interaction.accent_colour(&ctx).await;
            return interaction
                .respond(&ctx, |r| {
                    r.embed(|embed| {
                        embed
                            .author(|author| {
                                author
                                    .name(punished_user.member_name(&interaction.guild_id))
                                    .icon_url(punished_user.avatar())
                            })
                            .description(warnings_formatted)
                            .footer(|footer| {
                                footer.text(format!("User has a total of {} warnings.", punished_user.warnings.len()))
                            })
                            .colour(accent_colour)
                    })
                })
                .await;
        }

        let components = vec![
            Component::ActionRow(ActionRow {
                components: vec![Component::TextInput(TextInput {
                    custom_id: "mod-warn-text".to_owned(),
                    label: "Why they should be warned".to_owned(),
                    max_length: Some(2048),
                    min_length: Some(20),
                    placeholder: Some("They decided to shitpost in my DMs. This is utterly unacceptable.".to_owned()),
                    required: Some(true),
                    style: TextInputStyle::Paragraph,
                    value: None,
                })],
            }),
            Component::ActionRow(ActionRow {
                components: vec![Component::TextInput(TextInput {
                    custom_id: "mod-warn-id".to_owned(),
                    label: "User ID".to_owned(),
                    max_length: Some(20),
                    min_length: Some(8),
                    placeholder: None,
                    required: Some(true),
                    style: TextInputStyle::Short,
                    value: Some(punished_user.id.to_string()),
                })],
            }),
        ];

        interaction
            .respond(&ctx, |response| {
                response
                    .title("Add your warning below!")
                    .custom_id("mod-warn")
                    .add_components(components)
                    .response_type(InteractionResponseType::Modal)
            })
            .await
    }
}
