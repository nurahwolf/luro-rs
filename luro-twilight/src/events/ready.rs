use luro_framework::{Luro, LuroContext};
use luro_model::types::User;
use luro_model::{BOT_OWNERS, PRIMARY_BOT_OWNER, types::UserPermissions};
use tracing::{info, warn};
use twilight_model::gateway::{
    payload::{incoming::Ready, outgoing::UpdatePresence},
    presence::{ActivityType, MinimalActivity, Status},
};

use crate::commands::default_commands;

pub async fn ready_listener(framework: LuroContext, event: Box<Ready>) -> anyhow::Result<()> {
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

    let mut staff = framework.database.user_fetch_staff().await?;

    // let staff = vec![];

    // if staff.is_empty() {
    //     info!("-- Registering Staff in DB --");

    //     // Register staff
    //     for staff in BOT_OWNERS {
    //         let mut user = framework.fetch_user(staff).await?;
    //         if let Some(ref mut user_data) = user.data {
    //             user_data.permissions = UserPermissions::Administrator;
    //             framework.database.user_update_permissions(user_data).await?;
    //         }
    //     }

    //     // Register primary owner
    //     let mut owner = framework.fetch_user(PRIMARY_BOT_OWNER).await?;
    //     if let Some(ref mut user_data) = owner.data {
    //         user_data.permissions = UserPermissions::Owner;
    //         framework.database.user_update_permissions(user_data).await?;
    //     }

    //     staff = framework.database.user_fetch_staff().await?;
    // }

    if let Err(why) = framework.database.application_update(event.application.clone()).await {
        warn!("Heads up, failed to write application data to the database: {why}")
    };

    #[cfg(not(feature = "pretty-tables"))]
    standard_output(&framework, &event, staff).await;

    #[cfg(feature = "pretty-tables")]
    pretty_output(&framework, &event, staff).await;

    framework.register_commands(&default_commands()).await
}

#[cfg(not(feature = "pretty-tables"))]
async fn standard_output(framework: &LuroContext, event: &Box<Ready>, staff: HashMap<Id<UserMarker>, LuroUser>) {
    info!("Username:       {}", event.user.name);
    info!("ID:             {}", event.user.id);
    info!("Guilds:         {}", event.guilds.len());
    info!("API Version:    {}", event.version);
    if let Some(latency) = framework.latency.average() {
        info!("Latency:        {} ms", latency.as_millis());
    }
}

#[cfg(feature = "pretty-tables")]
async fn pretty_output(framework: &LuroContext, event: &Ready, staff: Vec<User>) {
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
    let mut administrators = String::new();
    let mut database_information = String::new();
    let mut owners = String::new();
    for staff in staff {
        if let Some(ref data) = staff.data {
            match data.permissions {
                UserPermissions::Owner => match owners.is_empty() {
                    true => owners.push_str(&staff.name()),
                    false => owners.push_str(format!(", {}", &staff.name()).as_str()),
                },
                UserPermissions::Administrator => match administrators.is_empty() {
                    true => administrators.push_str(&staff.name()),
                    false => administrators.push_str(format!(", {}", &staff.name()).as_str()),
                },
                _ => warn!("User {:#?} is tagged as a regular user in the database!", &staff),
            }
        }
    }

    builder = tabled::builder::Builder::new();
    builder.push_record(["Users with Owner Permission", &owners]);
    builder.push_record(["Users with Administrator Permission", &administrators]);
    if let Ok(data) = framework.database.driver.count_applications().await {
        builder.push_record(["Total Applications", &format_number(data)]);
    }
    if let Ok(data) = framework.database.driver.count_channels().await {
        builder.push_record(["Total Channels", &format_number(data)]);
    }
    if let Ok(data) = framework.database.driver.count_interactions().await {
        builder.push_record(["Total Interactions", &format_number(data)]);
    }
    if let Ok(data) = framework.database.driver.count_messages().await {
        builder.push_record(["Total Messages", &format_number(data.total_messages.unwrap_or_default())]);
    }
    database_information.push_str("-- General --\n");
    database_information.push_str(&builder.build().with(tabled::settings::Style::ascii_rounded()).to_string());
    builder = tabled::builder::Builder::new();

    // Guild Data
    if let Ok(data) = framework.database.driver.count_guilds().await {
        builder.push_record(["Total Guilds", &format_number(data)]);
    }
    if let Ok(data) = framework.database.driver.count_guild_members().await {
        builder.push_record(["Total Guild Members", &format_number(data)]);
    }
    if let Ok(data) = framework.database.driver.count_guild_roles().await {
        builder.push_record(["Total Guild Roles", &format_number(data)]);
    }
    database_information.push_str("\n-- Guild --\n");
    database_information.push_str(&builder.build().with(tabled::settings::Style::ascii_rounded()).to_string());
    builder = tabled::builder::Builder::new();

    // User Data
    if let Ok(data) = framework.database.driver.count_users().await {
        builder.push_record(["Total Users", &format_number(data)]);
    }
    if let Ok(data) = framework.database.driver.count_user_characters().await {
        builder.push_record(["Total User Characters", &format_number(data)]);
    }
    if let Ok(data) = framework.database.driver.count_user_marriages().await {
        builder.push_record(["Total User Marriages", &format_number(data)]);
    }
    if let Ok(data) = framework.database.driver.count_user_moderation_actions().await {
        builder.push_record(["Total User Moderation Actions", &format_number(data)]);
    }
    if let Ok(data) = framework.database.driver.count_user_warnings().await {
        builder.push_record(["Total User Warnings", &format_number(data)]);
    }
    database_information.push_str("\n-- User --\n");
    database_information.push_str(&builder.build().with(tabled::settings::Style::ascii_rounded()).to_string());

    info!("-- Database Information -- \n{}", database_information)
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
