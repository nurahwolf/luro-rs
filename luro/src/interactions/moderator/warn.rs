use crate::interaction::LuroSlash;
use luro_model::database::drivers::LuroDatabaseDriver;

use twilight_model::http::interaction::InteractionResponseType;

use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

use twilight_model::channel::message::component::{ActionRow, TextInput, TextInputStyle};
use twilight_model::channel::message::Component;
use twilight_model::guild::Permissions;

use crate::luro_command::LuroCommand;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(
    name = "warn",
    desc = "Get the warnings of a user, or set a new warning",
    default_permissions = "Self::default_permissions"
)]
pub struct ModeratorWarnCommand {
    /// Want to make a new warning?
    new: bool,
    /// The user to warn.
    user: ResolvedUser
}

impl LuroCommand for ModeratorWarnCommand {
    fn default_permissions() -> Permissions {
        Permissions::MANAGE_MESSAGES
    }

    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        let user_id = self.user.resolved.id;

        if !self.new {
            let user_data = ctx.framework.database.get_user(&user_id, &ctx.framework.twilight_client).await?;
            if user_data.warnings.is_empty() {
                return ctx.respond(|r| r.content("No warnings for that user!")).await;
            }

            let luro_user = ctx.framework.database.get_user(&ctx.interaction.author_id().unwrap(), &ctx.framework.twilight_client).await?;
            let mut warnings_formatted = String::new();
            for (warning, user_id) in &user_data.warnings {
                writeln!(warnings_formatted, "Warning by <@{user_id}>```{warning}```")?
            }

            let accent_colour = ctx.accent_colour().await;
            return ctx
                .respond(|r| {
                    r.embed(|embed| {
                        embed
                            .author(|author| author.name(luro_user.name()).icon_url(luro_user.avatar()))
                            .description(warnings_formatted)
                            .footer(|footer| footer.text(format!("User has a total of {} warnings.", user_data.warnings.len())))
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
                    value: None
                })]
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
                    value: Some(user_id.to_string())
                })]
            }),
        ];

        ctx.respond(|response| {
            response
                .title("Add your warning below!")
                .custom_id("mod-warn")
                .add_components(components)
                .response_type(InteractionResponseType::Modal)
        })
        .await
    }
}
