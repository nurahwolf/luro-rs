use crate::traits::luro_functions::LuroFunctions;
use crate::USERDATA_FILE_PATH;
use anyhow::Context;
use luro_model::{user_actions::UserActions, user_actions_type::UserActionType};

use async_trait::async_trait;
use std::convert::TryInto;
use std::fmt::Write;
use std::path::Path;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use twilight_model::application::interaction::modal::ModalInteractionData;
use twilight_model::channel::message::component::{ActionRow, TextInput, TextInputStyle};
use twilight_model::channel::message::Component;
use twilight_model::guild::Permissions;
use twilight_model::id::marker::UserMarker;
use twilight_model::id::Id;
use twilight_util::builder::embed::{EmbedAuthorBuilder, ImageSource};
use twilight_util::builder::embed::{EmbedFieldBuilder, EmbedFooterBuilder};

use crate::models::{LuroSlash, SlashUser, UserData};

use crate::traits::luro_command::LuroCommand;
use crate::traits::toml::LuroTOML;
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

#[async_trait]
impl LuroCommand for ModeratorWarnCommand {
    fn default_permissions() -> Permissions {
        Permissions::MANAGE_MESSAGES
    }

    async fn run_command(self, mut ctx: LuroSlash) -> anyhow::Result<()> {
        let user_id = self.user.resolved.id;

        if !self.new {
            let user_data = UserData::get_user_settings(&ctx.luro, &user_id).await?;
            if user_data.warnings.is_empty() {
                return ctx.content("No warnings for that user!").respond().await;
            }

            let slash_author = SlashUser::client_fetch_user(&ctx.luro, user_id).await?.1;
            let embed_author = EmbedAuthorBuilder::new(&slash_author.name).icon_url(slash_author.try_into()?);
            let mut warnings_formatted = String::new();
            for (warning, user_id) in &user_data.warnings {
                writeln!(warnings_formatted, "Warning by <@{user_id}>```{warning}```")?
            }

            let embed = ctx
                .luro
                .default_embed(&ctx.interaction.guild_id)
                .author(embed_author)
                .description(warnings_formatted)
                .footer(EmbedFooterBuilder::new(format!(
                    "User has a total of {} warnings.",
                    user_data.warnings.len()
                )));
            return ctx.embed(embed.build())?.respond().await;
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

        ctx.custom_id("mod-warn".to_owned())
            .title("Add your warning below!".to_owned())
            .components(components)
            .model()
            .respond()
            .await
    }

    async fn handle_model(data: ModalInteractionData, mut ctx: LuroSlash) -> anyhow::Result<()> {
        ctx.deferred_component().await?;
        let author = ctx.author()?;
        let warning = ctx.parse_modal_field_required(&data, "mod-warn-text")?;
        let id = ctx.parse_modal_field_required(&data, "mod-warn-id")?;
        let user_id: Id<UserMarker> = Id::new(id.parse::<u64>()?);
        let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, user_id);

        let slash_author = SlashUser::client_fetch(&ctx.luro, ctx.interaction.guild_id, author.id).await?;

        let mut embed = ctx.luro.default_embed(&ctx.interaction.guild_id);
        let embed_author = EmbedAuthorBuilder::new(format!("Warning by {}", slash_author.name))
            .icon_url(ImageSource::url(slash_author.avatar)?)
            .build();
        embed = embed
            .author(embed_author)
            .description(format!("Warning Created for <@{user_id}>\n```{warning}```"));

        {
            let mut user_data = UserData::modify_user_settings(&ctx.luro, &user_id).await?;
            user_data.warnings.push((warning.to_owned(), author.id));

            ctx.luro.user_data.insert(user_id, user_data.clone());
            user_data.write(Path::new(&path)).await?;

            embed = embed.footer(EmbedFooterBuilder::new(format!(
                "User has a total of {} warnings.",
                user_data.warnings.len()
            )));
        }

        match ctx.luro.twilight_client.create_private_channel(user_id).await {
            Ok(channel) => {
                let channel = channel.model().await?;
                let victim_dm = ctx
                    .luro
                    .twilight_client
                    .create_message(channel.id)
                    .embeds(&[embed.clone().build()])
                    .await;
                match victim_dm {
                    Ok(_) => embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Successful").inline()),
                    Err(_) => embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Failed").inline())
                }
            }
            Err(_) => embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Failed").inline())
        };

        let response = ctx.embed(embed.clone().build())?.respond().await;

        ctx.luro.send_moderator_log_channel(&ctx.interaction.guild_id, embed).await?;

        {
            let mut reward = UserData::modify_user_settings(&ctx.luro, &author.id).await?;
            let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &author.id);
            reward.moderation_actions_performed += 1;
            reward.write(Path::new(&path)).await?;
        }

        {
            // Record the punishment
            let mut warned = UserData::modify_user_settings(&ctx.luro, &author.id).await?;
            let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &author.id);
            warned.moderation_actions.push(UserActions {
                action_type: vec![UserActionType::Warn],
                guild_id: ctx.interaction.guild_id,
                reason: warning.to_owned(),
                responsible_user: author.id
            });
            warned.write(Path::new(&path)).await?;
        }

        response
    }
}