use poise::serenity_prelude::{Activity, Context, OnlineStatus, Ready};

use crate::Error;

/// A Serenity listener for the [Ready] type
pub async fn ready_listener(ready: &Ready, ctx: &Context) -> Result<(), Error> {
    let http = &ctx.http;
    let api_version = ready.version;
    let bot_gateway = http.get_bot_gateway().await.unwrap();
    let t_sessions = bot_gateway.session_start_limit.total;
    let r_sessions = bot_gateway.session_start_limit.remaining;

    println!("Successfully logged into Discord as the following user:");
    println!("Bot username: {}", ready.user.tag());
    println!("Bot user ID: {}", ready.user.id);
    if let Ok(application_info) = http.get_current_application_info().await {
        println!("Bot owner: {}", application_info.owner.tag());
    }

    let guild_count = ready.guilds.len();

    println!("Connected to the Discord API (version {api_version}) with {r_sessions}/{t_sessions} sessions remaining.");
    println!("Connected to and serving a total of {guild_count} guild(s).");

    let presence_string = format!("on {guild_count} guilds | @luro help");
    ctx.set_presence(Some(Activity::playing(&presence_string)), OnlineStatus::Online)
        .await;
    Ok(())
}
