use twilight_gateway::MessageSender;
use twilight_model::gateway::payload::incoming::Ready;

use crate::gateway::{GatewayArc, GatewayResult};

pub async fn ready_listener(gateway: GatewayArc, shard: MessageSender, event: Box<Ready>) -> GatewayResult {
    tracing::info!("Luro is now ready!");
    tracing::info!("==================");

    let presence_string = format!(
        "/about | on {} guilds | shard {}",
        event.guilds.len(),
        event.shard.map(|x| x.number()).unwrap_or(0)
    );

    let presence = twilight_model::gateway::payload::outgoing::UpdatePresence::new(
        vec![twilight_model::gateway::presence::MinimalActivity {
            kind: twilight_model::gateway::presence::ActivityType::Playing,
            name: presence_string,
            url: None,
        }
        .into()],
        false,
        None,
        twilight_model::gateway::presence::Status::Online,
    );

    match presence {
        Ok(presence) => {
            if let Err(why) = shard.command(&presence) {
                tracing::info!(?why, "Failed to update presence")
            }
        }
        Err(why) => tracing::info!(?why, "Failed to create new presence object"),
    }

    // let mut staff = framework.database.user_fetch_staff().await?;

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

    #[cfg(not(feature = "pretty-tables"))]
    standard_output(&event).await;

    #[cfg(feature = "pretty-tables")]
    pretty_output(&framework, &event, staff).await;

    #[cfg(feature = "module-interactions")]
    gateway.register_commands(&crate::commands::default_commands()).await
}

#[cfg(not(feature = "pretty-tables"))]
async fn standard_output(event: &Ready) {
    tracing::info!("Username:       {}", event.user.name);
    tracing::info!("ID:             {}", event.user.id);
    tracing::info!("Guilds:         {}", event.guilds.len());
    tracing::info!("API Version:    {}", event.version);
    // if let Some(latency) = ctx.data.latency.average() {
    //     tracing::info!("Latency:        {} ms", latency.as_millis());
    // }
}

#[cfg(feature = "pretty-tables")]
async fn pretty_output(framework: &LuroContext, event: &Ready, staff: Vec<User>) {
    use tabled::settings::Style;
    use thousands::Separable;

    let mut builder = tabled::builder::Builder::new();
    builder.set_header([
        "API Latency",
        "API Version",
        "Bot ID",
        "Bot Username",
        "Total Guilds",
        "Total Shards",
    ]);
    builder.push_record([
        match framework.latency.average() {
            Some(latency) => latency.as_millis().to_string(),
            None => "None".to_owned(),
        },
        event.version.to_string(),
        event.user.id.to_string(),
        event.user.name.clone(),
        event.guilds.len().to_string(),
        match event.shard {
            Some(shard) => shard.total().to_string(),
            None => "Unknown".to_owned(),
        },
    ]);

    match event.shard {
        Some(shard) => {
            info!(
                "-- Bot Information | Shard {} -- \n{}",
                shard.number(),
                builder.build().with(Style::sharp()).to_string()
            )
        }
        None => info!(
            "-- Bot Information -- \n{}",
            builder.build().with(tabled::settings::Style::sharp()).to_string()
        ),
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
                    false => owners.push_str(format!("\n{}", &staff.name()).as_str()),
                },
                UserPermissions::Administrator => match administrators.is_empty() {
                    true => administrators.push_str(&staff.name()),
                    false => administrators.push_str(format!("\n{}", &staff.name()).as_str()),
                },
                _ => warn!("User {:#?} is tagged as a regular user in the database!", &staff),
            }
        }
    }

    builder = tabled::builder::Builder::new();
    builder.set_header([
        "Administrators",
        "Owners",
        "Applications",
        "Channels",
        "Interactions",
        "Messages",
        "Guilds",
        "Members",
        "Roles",
        "Users",
        "Characters",
        "Marriages",
    ]);
    builder.push_record([
        administrators,
        owners,
        match framework.database.sqlx.count_applications().await {
            Ok(data) => data.separate_with_commas(),
            Err(_) => "Unknown".to_owned(),
        },
        match framework.database.sqlx.count_channels().await {
            Ok(data) => data.separate_with_commas(),
            Err(_) => "Unknown".to_owned(),
        },
        match framework.database.sqlx.count_interactions().await {
            Ok(data) => data.separate_with_commas(),
            Err(_) => "Unknown".to_owned(),
        },
        match framework.database.sqlx.count_messages().await {
            Ok(data) => data.total_messages.unwrap_or_default().separate_with_commas(),
            Err(_) => "Unknown".to_owned(),
        },
        match framework.database.sqlx.count_guilds().await {
            Ok(data) => data.separate_with_commas(),
            Err(_) => "Unknown".to_owned(),
        },
        match framework.database.sqlx.count_guild_members().await {
            Ok(data) => data.separate_with_commas(),
            Err(_) => "Unknown".to_owned(),
        },
        match framework.database.sqlx.count_guild_roles().await {
            Ok(data) => data.separate_with_commas(),
            Err(_) => "Unknown".to_owned(),
        },
        match framework.database.sqlx.count_users().await {
            Ok(data) => data.separate_with_commas(),
            Err(_) => "Unknown".to_owned(),
        },
        match framework.database.sqlx.count_user_characters().await {
            Ok(data) => data.separate_with_commas(),
            Err(_) => "Unknown".to_owned(),
        },
        match framework.database.sqlx.count_user_marriages().await {
            Ok(data) => data.separate_with_commas(),
            Err(_) => "Unknown".to_owned(),
        },
    ]);
    database_information.push_str(&builder.build().with(tabled::settings::Style::sharp()).to_string());
    info!("-- Database Information -- \n{}", database_information)
}
