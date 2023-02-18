use std::path::Path;

use anyhow::Result;
use git2::{ErrorCode, Repository};
use memory_stats::memory_stats;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::Interaction,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder};

use crate::Luro;

/// Retrieves the current git branch in a given git repository.
fn get_current_branch(repo: &Repository) -> String {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
            None
        }
        Err(e) => return format!("An error occured: {e:?}"),
    };

    let head = head.as_ref().and_then(|h| h.shorthand());
    head.unwrap().to_string()
}

/// Retrieves the latest HEAD revision for the given git repository.
fn get_head_revision(repo: &Repository) -> String {
    let revspec = repo.revparse("HEAD").unwrap();
    let revision = revspec.from().unwrap();
    revision.short_id().unwrap().as_str().unwrap().to_string()
}

/// About the bot!
#[derive(CommandModel, CreateCommand)]
#[command(
    name = "about",
    desc = "Information about me, such as my creator and my source code!"
)]
pub struct AboutCommand {}

pub async fn about_command<'a>(luro: &Luro, interaction: &Interaction) -> Result<()> {
    let accent_colour = luro.accent_colour(interaction.guild_id).await;
    let repo = Repository::open(Path::new(env!("CARGO_MANIFEST_DIR")).join("."))?;
    let version = env!("CARGO_PKG_VERSION").to_string();
    let branch = get_current_branch(&repo);
    let revision = get_head_revision(&repo);
    let application_owner = luro
        .http
        .current_user_application()
        .await
        .unwrap()
        .model()
        .await
        .unwrap()
        .owner
        .unwrap();

    let usage = match memory_stats() {
        Some(ok) => ok,
        None => panic!("Could not get memory stats!"),
    };

    let embed = EmbedBuilder::default()
        .title("About Me!")
        .description("Some information about me...")
        .color(accent_colour)
        .field(EmbedFieldBuilder::new("Version", version).inline())
        .field(EmbedFieldBuilder::new("Branch", branch).inline())
        .field(EmbedFieldBuilder::new("Revision", format!("`{revision}`")).inline())
        .field(
            EmbedFieldBuilder::new(
                "Main Bot Owner",
                format!(
                    "<@{}> ({}#{})",
                    application_owner.id, application_owner.name, application_owner.discriminator
                ),
            )
            .inline(),
        )
        .field(
            EmbedFieldBuilder::new(
                "Physical memory usage",
                format!("{}MB", usage.physical_mem / 1024 / 1024),
            )
            .inline(),
        )
        .field(
            EmbedFieldBuilder::new(
                "Virtual memory usage",
                format!("{}MB", usage.virtual_mem / 1024 / 1024),
            )
            .inline(),
        );

    let embeds = vec![embed.build()];

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            embeds: Some(embeds),
            ..Default::default()
        }),
    };

    match luro
        .http
        .interaction(luro.application_id)
        .create_response(interaction.id, &interaction.token, &response)
        .await
    {
        Ok(ok) => ok,
        Err(_) => todo!(),
    };

    Ok(())
}
