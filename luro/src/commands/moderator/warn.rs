use crate::interaction::LuroSlash;

use anyhow::Context;
use luro_builder::embed::EmbedBuilder;
use luro_model::luro_log_channel::LuroLogChannel;
use luro_model::{user_actions::UserActions, user_actions_type::UserActionType};
use twilight_model::http::interaction::InteractionResponseType;

use std::fmt::Write;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::application::interaction::modal::ModalInteractionData;
use twilight_model::channel::message::component::{ActionRow, TextInput, TextInputStyle};
use twilight_model::channel::message::Component;
use twilight_model::guild::Permissions;
use twilight_model::id::marker::UserMarker;
use twilight_model::id::Id;

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

    async fn run_command(self, ctx: LuroSlash) -> anyhow::Result<()> {
        let user_id = self.user.resolved.id;

        if !self.new {
            let user_data = ctx.framework.database.get_user(&user_id).await?;
            if user_data.warnings.is_empty() {
                return ctx.respond(|r| r.content("No warnings for that user!")).await;
            }

            let luro_user = ctx.framework.database.get_user(&ctx.interaction.author_id().unwrap()).await?;
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

    async fn handle_model(data: ModalInteractionData, ctx: LuroSlash) -> anyhow::Result<()> {
        let author = ctx.interaction.author().context("Expected to get interaction author")?;
        let warning = ctx.parse_modal_field_required(&data, "mod-warn-text")?;
        let id = ctx.parse_modal_field_required(&data, "mod-warn-id")?;
        let user_id: Id<UserMarker> = Id::new(id.parse::<u64>()?);

        let luro_user = ctx.framework.database.get_user(&ctx.interaction.author_id().unwrap()).await?;

        let mut user_data = ctx.framework.database.get_user(&user_id).await?;
        user_data.warnings.push((warning.to_owned(), author.id));
        ctx.framework.database.modify_user(&user_id, &user_data).await?;

        let mut embed = EmbedBuilder::default();
        embed
            .description(format!("Warning Created for <@{user_id}>\n```{warning}```"))
            .colour(ctx.accent_colour().await)
            .footer(|footer| footer.text(format!("User has a total of {} warnings.", user_data.warnings.len())))
            .author(|author| {
                author
                    .name(format!("Warning by {}", luro_user.name()))
                    .icon_url(luro_user.avatar())
            });

        match ctx.framework.twilight_client.create_private_channel(user_id).await {
            Ok(channel) => {
                let channel = channel.model().await?;
                let victim_dm = ctx
                    .framework
                    .twilight_client
                    .create_message(channel.id)
                    .embeds(&[embed.clone().into()])
                    .await;
                match victim_dm {
                    Ok(_) => embed.create_field("DM Sent", "Successful", true),
                    Err(_) => embed.create_field("DM Sent", "Failed", true)
                }
            }
            Err(_) => embed.create_field("DM Sent", "Failed", true)
        };

        ctx.send_log_channel(LuroLogChannel::Moderator, |r| r.add_embed(embed.clone()))
            .await?;

        let mut reward = ctx.framework.database.get_user(&author.id).await?;
        reward.moderation_actions_performed += 1;
        ctx.framework.database.modify_user(&author.id, &reward).await?;

        // Record the punishment
        let mut warned = ctx.framework.database.get_user(&user_id).await?;
        warned.moderation_actions.push(UserActions {
            action_type: vec![UserActionType::Warn],
            guild_id: ctx.interaction.guild_id,
            reason: warning.to_owned(),
            responsible_user: author.id
        });
        ctx.framework.database.modify_user(&user_id, &warned).await?;

        ctx.respond(|response| response.add_embed(embed)).await
    }
}
