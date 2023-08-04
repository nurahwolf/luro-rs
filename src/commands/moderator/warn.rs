use crate::traits::luro_functions::LuroFunctions;
use crate::USERDATA_FILE_PATH;
use anyhow::Context;
use async_trait::async_trait;
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

use crate::models::{LuroSlash, UserActionType, UserActions, UserData};

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
            let warnings = UserData::get_user_settings(&ctx.luro, &user_id).await?.warnings;
            if warnings.is_empty() {
                return ctx.clone().content("No warnings for that user!").respond().await;
            }

            let user = ctx.luro.twilight_client.user(user_id).await?.model().await?;
            let avatar = ctx.user_get_avatar(&user);
            let author = EmbedAuthorBuilder::new(user.name).icon_url(ImageSource::url(avatar)?);
            let mut warnings_formatted = String::new();
            for (warning, user_id) in &warnings {
                writeln!(warnings_formatted, "Warning by <@{user_id}>```{warning}```")?
            }

            let embed = ctx
                .luro
                .default_embed(&ctx.interaction.guild_id)
                .author(author)
                .description(warnings_formatted)
                .footer(EmbedFooterBuilder::new(format!(
                    "User has a total of {} warnings.",
                    warnings.len()
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
        let author = ctx.author()?;
        let warning = ctx.parse_modal_field_required(&data, "mod-warn-text")?;
        let id = ctx.parse_modal_field_required(&data, "mod-warn-id")?;
        let user_id: Id<UserMarker> = Id::new(id.parse::<u64>()?);
        let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, user_id);
        let path = Path::new(&path);
        let user = ctx.luro.twilight_client.user(ctx.author()?.id).await?.model().await?;

        let mut embed = ctx.luro.default_embed(&ctx.interaction.guild_id);
        let avatar = ctx.user_get_avatar(&user);
        let embed_author = EmbedAuthorBuilder::new(format!("Warning by {}", user.name))
            .icon_url(ImageSource::url(avatar)?)
            .build();
        embed = embed
            .author(embed_author)
            .description(format!("Warning Created for <@{user_id}>\n```{warning}```"));

        let mut user_data = UserData::get_user_settings(&ctx.luro, &user_id).await?;
        user_data.warnings.push((warning.to_owned(), author.id));

        ctx.luro.user_data.insert(user_id, user_data.clone());
        user_data.write(path).await?;

        embed = embed.footer(EmbedFooterBuilder::new(format!(
            "User has a total of {} warnings.",
            user_data.warnings.len()
        )));

        match ctx.luro.twilight_client.create_private_channel(user_id).await {
            Ok(channel) => {
                let channel = channel.model().await?;
                let victim_dm = ctx
                    .luro
                    .twilight_client
                    .create_message(channel.id)
                    .embeds(&[embed.clone().build()])?
                    .await;
                match victim_dm {
                    Ok(_) => embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Successful").inline()),
                    Err(_) => embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Failed").inline())
                }
            }
            Err(_) => embed = embed.field(EmbedFieldBuilder::new("DM Sent", "Failed").inline())
        };

        ctx.luro
            .send_moderator_log_channel(&ctx.interaction.guild_id, embed.clone())
            .await?;

        {
            let _ = UserData::get_user_settings(&ctx.luro, &author.id).await?;
            let _ = UserData::get_user_settings(&ctx.luro, &user_id).await?;
            // Reward the person who actioned the ban
            let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &author.id);
            let data = &mut ctx
                .luro
                .user_data
                .get_mut(&author.id)
                .context("Expected to find user's data in the cache")?;
            data.moderation_actions_performed += 1;
            data.write(Path::new(&path)).await?;
            // Record the punishment
            let path = format!("{0}/{1}/user_settings.toml", USERDATA_FILE_PATH, &user_id);
            let data = &mut ctx
                .luro
                .user_data
                .get_mut(&user_id)
                .context("Expected to find user's data in the cache")?;
            data.moderation_actions.push(UserActions {
                action_type: vec![UserActionType::Warn],
                guild_id: Some(ctx.interaction.guild_id.context("Expected this to be a guild")?),
                reason: warning.to_owned(),
                responsible_user: author.id
            });
            data.write(Path::new(&path)).await?;
        }

        ctx.embed(embed.build())?.respond().await
    }
}
