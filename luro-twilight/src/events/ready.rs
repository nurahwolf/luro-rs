use luro_framework::{Context, Luro};
use luro_model::{BOT_OWNERS, PRIMARY_BOT_OWNER};
use std::collections::HashMap;
use tracing::{info, warn};
use twilight_model::gateway::{
    payload::{incoming::Ready, outgoing::UpdatePresence},
    presence::{ActivityType, MinimalActivity, Status},
};

use luro_model::user::LuroUser;
use twilight_model::id::{marker::UserMarker, Id};

use crate::commands::default_commands;

pub async fn ready_listener(framework: Context, event: Box<Ready>) -> anyhow::Result<()> {
    info!("Luro is now ready!");
    info!("==================");

    if let Some(shard_id) = event.shard {
        let presence_string = format!("/about | on {} guilds | shard {}", event.guilds.len(), shard_id.number());

        framework.shard.command(&UpdatePresence::new(
            vec![MinimalActivity {
                kind: ActivityType::Playing,
                name: presence_string,
                url: None,
            }
            .into()],
            false,
            None,
            Status::Online,
        )?)?;
    };

    let mut staff = framework.database.get_staff().await;

    if staff.is_empty() {
        info!("-- Registering Staff in DB --");

        // Register staff
        for staff in BOT_OWNERS {
            framework
                .database
                .register_staff(framework.twilight_client.user(staff).await?.model().await?)
                .await?;
        }

        // Register primary owner
        framework
            .database
            .register_owner(framework.twilight_client.user(PRIMARY_BOT_OWNER).await?.model().await?)
            .await?;

        staff = framework.database.get_staff().await;
    }

    #[cfg(not(feature = "pretty-tables"))]
    standard_output(&framework, &event, staff).await;

    #[cfg(feature = "pretty-tables")]
    pretty_output(&framework, &event, staff).await;

    framework.register_commands(&default_commands()).await
}

#[cfg(not(feature = "pretty-tables"))]
async fn standard_output(framework: &Context, event: &Box<Ready>, staff: HashMap<Id<UserMarker>, LuroUser>) {
    info!("Username:       {}", event.user.name);
    info!("ID:             {}", event.user.id);
    info!("Guilds:         {}", event.guilds.len());
    info!("API Version:    {}", event.version);
    if let Some(latency) = framework.latency.average() {
        info!("Latency:        {} ms", latency.as_millis());
    }
}

#[cfg(feature = "pretty-tables")]
async fn pretty_output(framework: &Context, event: &Ready, staff: HashMap<Id<UserMarker>, LuroUser>) {
    let mut builder = tabled::builder::Builder::new();
    if let Some(latency) = framework.latency.average() {
        builder.push_record(["API Latency", &latency.as_millis().to_string()]);
    }
    builder.push_record(["API Version", &event.version.to_string()]);
    builder.push_record(["Bot ID", &event.user.id.to_string()]);
    builder.push_record(["Bot Username", &event.user.name]);
    builder.push_record(["Total Guilds", &event.guilds.len().to_string()]);

    match event.shard {
        Some(shard) => {
            builder.push_record(["Total Shards", &shard.total().to_string()]);

            info!(
                "-- Bot Information | Shard {} -- \n{}",
                shard.number(),
                builder.build().with(tabled::settings::Style::ascii_rounded()).to_string()
            )
        }
        None => info!(
            "-- Bot Information -- \n{}",
            builder.build().with(tabled::settings::Style::ascii_rounded()).to_string()
        ),
    }

    // Information from Discord's API
    if let Ok(application) = framework.twilight_client.current_user_application().await {
        if let Ok(application) = application.model().await {
            builder = tabled::builder::Builder::new();

            if let Some(owner) = application.owner {
                builder.push_record([
                    "Application Owner",
                    &format!("{} - {}", owner.global_name.unwrap_or(owner.name), owner.id),
                ]);
            }

            info!(
                "-- Application Information -- \n{}",
                builder.build().with(tabled::settings::Style::ascii_rounded()).to_string()
            )
        }
    }

    // Information from the Database
    let mut owners = String::new();
    let mut administrators = String::new();
    for staff in staff.values() {
        match staff.user_permissions {
            luro_model::user::LuroUserPermissions::Owner => match owners.is_empty() {
                true => owners.push_str(&staff.name),
                false => owners.push_str(format!(", {}", staff.name).as_str()),
            },
            luro_model::user::LuroUserPermissions::Administrator => match administrators.is_empty() {
                true => administrators.push_str(&staff.name),
                false => administrators.push_str(format!(", {}", staff.name).as_str()),
            },
            _ => warn!("User {:#?} is tagged as a regular user in the database!", staff),
        }
    }

    builder = tabled::builder::Builder::new();
    builder.push_record(["Users with Owner Permission", &owners]);
    builder.push_record(["Users with Administrator Permission", &administrators]);
    if let Ok(data) = framework.database.count_users().await {
        builder.push_record([
            "Total Users",
            &data
                .to_string()
                .as_bytes()
                .rchunks(3)
                .rev()
                .map(std::str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap()
                .join(","),
        ]);
    }
    if let Ok(data) = framework.database.count_guilds().await {
        builder.push_record([
            "Total Guilds",
            &data
                .to_string()
                .as_bytes()
                .rchunks(3)
                .rev()
                .map(std::str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap()
                .join(","),
        ]);
    }
    if let Ok(data) = framework.database.count_interactions().await {
        builder.push_record([
            "Total Interactions",
            &data
                .to_string()
                .as_bytes()
                .rchunks(3)
                .rev()
                .map(std::str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap()
                .join(","),
        ]);
    }
    if let Ok(data) = framework.database.count_messages().await {
        builder.push_record([
            "Total Messages",
            &data
                .to_string()
                .as_bytes()
                .rchunks(3)
                .rev()
                .map(std::str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap()
                .join(","),
        ]);
    }
    if let Ok(data) = framework.database.count_channels().await {
        builder.push_record([
            "Total Channels",
            &data
                .to_string()
                .as_bytes()
                .rchunks(3)
                .rev()
                .map(std::str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap()
                .join(","),
        ]);
    }
    if let Ok(data) = framework.database.count_roles().await {
        builder.push_record([
            "Total Roles",
            &data
                .to_string()
                .as_bytes()
                .rchunks(3)
                .rev()
                .map(std::str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap()
                .join(","),
        ]);
    }

    info!(
        "-- Database Information -- \n{}",
        builder.build().with(tabled::settings::Style::ascii_rounded()).to_string()
    )
}
