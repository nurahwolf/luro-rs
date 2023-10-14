use luro_database::LuroUserPermissions;
use luro_framework::{command::ExecuteLuroCommand, interactions::InteractionTrait, CommandInteraction};

use tracing::warn;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::http::interaction::InteractionResponseType;

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "database", desc = "Information about my database")]
pub struct Database {}

impl ExecuteLuroCommand for Database {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        ctx.acknowledge_interaction(false).await?;
        let mut embed = ctx.default_embed().await;

        let mut builder = tabled::builder::Builder::new();
        if let Some(latency) = ctx.latency.average() {
            builder.push_record(["API Latency", &latency.as_millis().to_string()]);
        }

        // Information from Discord's API
        if let Ok(application) = ctx.twilight_client.current_user_application().await {
            if let Ok(application) = application.model().await {
                builder = tabled::builder::Builder::new();

                if let Some(owner) = application.owner {
                    builder.push_record([
                        "Application Owner",
                        &format!("{} - {}", owner.global_name.unwrap_or(owner.name), owner.id),
                    ]);
                }

                embed.field(|f| {
                    f.field(
                        "-- Application Information --",
                        &format!("```\n{}```", builder.build().with(tabled::settings::Style::ascii_rounded())),
                        false,
                    )
                });
            }
        }

        // Information from the Database
        let mut administrators = String::new();
        let mut owners = String::new();
        for staff in ctx.database.get_staff().await? {
            match staff.user_permissions.as_ref().unwrap_or(&LuroUserPermissions::User) {
                LuroUserPermissions::Owner => match owners.is_empty() {
                    true => owners.push_str(&staff.name),
                    false => owners.push_str(format!(", {}", &staff.name).as_str()),
                },
                LuroUserPermissions::Administrator => match administrators.is_empty() {
                    true => administrators.push_str(&staff.name),
                    false => administrators.push_str(format!(", {}", &staff.name).as_str()),
                },
                _ => warn!("User {:#?} is tagged as a regular user in the database!", &staff),
            }
        }

        builder = tabled::builder::Builder::new();
        builder.push_record(["Users with Owner Permission", &owners]);
        builder.push_record(["Users with Administrator Permission", &administrators]);
        if let Ok(data) = ctx.database.count_applications().await {
            builder.push_record(["Total Applications", &format_number(data)]);
        }
        if let Ok(data) = ctx.database.count_channels().await {
            builder.push_record(["Total Channels", &format_number(data)]);
        }
        if let Ok(data) = ctx.database.count_interactions().await {
            builder.push_record(["Total Interactions", &format_number(data)]);
        }
        if let Ok(data) = ctx.database.count_messages().await {
            builder.push_record(["Total Messages", &format_number(data)]);
        }

        let content = format!("**-- General Information --**\n```\n{}```", builder.build().with(tabled::settings::Style::ascii_rounded()));
        builder = tabled::builder::Builder::new();

        // Guild Data
        if let Ok(data) = ctx.database.count_guilds().await {
            builder.push_record(["Total Guilds", &format_number(data)]);
        }
        if let Ok(data) = ctx.database.count_guild_channels().await {
            builder.push_record(["Total Guild Channels", &format_number(data)]);
        }
        if let Ok(data) = ctx.database.count_guild_roles().await {
            builder.push_record(["Total Guild Roles", &format_number(data)]);
        }
        if let Ok(data) = ctx.database.count_guild_members().await {
            builder.push_record(["Total Guild Members", &format_number(data)]);
        }
        if let Ok(data) = ctx.database.count_guild_roles().await {
            builder.push_record(["Total Guild Roles", &format_number(data)]);
        }
        embed.field(|f: &mut luro_model::builders::embed::embed_field::EmbedFieldBuilder| {
            f.field(
                "-- Guild Information --",
                &format!("```\n{}```", builder.build().with(tabled::settings::Style::ascii_rounded())),
                false,
            )
        });
        builder = tabled::builder::Builder::new();

        // User Data
        if let Ok(data) = ctx.database.count_users().await {
            builder.push_record(["Total Users", &format_number(data)]);
        }
        if let Ok(data) = ctx.database.count_user_characters().await {
            builder.push_record(["Total User Characters", &format_number(data)]);
        }
        if let Ok(data) = ctx.database.count_user_moderation_actions().await {
            builder.push_record(["Total User Moderation Actions", &format_number(data)]);
        }
        if let Ok(data) = ctx.database.count_user_warnings().await {
            builder.push_record(["Total User Warnings", &format_number(data)]);
        }
        embed.field(|f| {
            f.field(
                "-- User Information --",
                &format!("```\n{}```", builder.build().with(tabled::settings::Style::ascii_rounded())),
                false,
            )
        });

        ctx.respond(|response| {
            response
                .content(content)
                .add_embed(embed)
                .response_type(InteractionResponseType::DeferredChannelMessageWithSource)
        })
        .await
    }
}

fn format_number(input: i64) -> String {
    input
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",")
}
