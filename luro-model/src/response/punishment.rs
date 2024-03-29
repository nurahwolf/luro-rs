use std::time::{SystemTime, UNIX_EPOCH};

use twilight_model::util::Timestamp;

use crate::{
    builders::EmbedBuilder,
    emoji::{JOIN_EMOJI, LEAVE_EMOJI, MEMBER_EMOJI, PRIVATE_EMOJI, TICKET_EMOJI},
    guild::Guild,
    user::{MemberContext, User},
};

/// The type of punishment a user should receive.
pub enum Punishment<'a> {
    /// The user was banned. Also contains how many messages were purged.
    Banned(PunishmentData<'a>, i64),
    /// The user was unbanned.
    Unbanned(PunishmentData<'a>),
    /// The user received a warning.
    Warned(PunishmentData<'a>),
    /// The user was kicked.
    Kicked(PunishmentData<'a>),
}

/// Standard data used for all types of punishments.
pub struct PunishmentData<'a> {
    /// The person who performed the action
    pub author: &'a MemberContext,
    /// The person who received the punishment
    pub target: &'a User,
    /// The reason the action was performed
    pub reason: &'a str,
    /// Information relating to the guild in which they were actioned from
    pub guild: &'a Guild<'a>,
    /// Has the user been informed about their punishment. This is not set when the punishment is made, and set when we attempt to inform them.
    pub dm_successful: Option<bool>,
}

/// Useful functionality around the punishment's contained data
impl<'a> PunishmentData<'a> {
    /// Sets the nested DM success status to contain the passed value
    pub fn dm_successful(&mut self, success: bool) {
        self.dm_successful = Some(success);
    }
}

/// Functionality around the high-level punishment object
impl<'a> Punishment<'a> {
    /// Get a mutatable version of the contained punishment data. Useful for updating it.
    pub fn data(&mut self) -> &mut PunishmentData<'a> {
        match self {
            Punishment::Banned(data, _) => data,
            Punishment::Unbanned(data) => data,
            Punishment::Warned(data) => data,
            Punishment::Kicked(data) => data,
        }
    }

    /// Convert the contained data into an embed builder
    pub fn embed(&self) -> EmbedBuilder {
        match self {
            Punishment::Banned(data, purged_message_seconds) => ban_embed(data, purged_message_seconds),
            Punishment::Unbanned(data) => unban_embed(data),
            Punishment::Warned(data) => warn_embed(data),
            Punishment::Kicked(data) => kick_embed(data),
        }
    }
}

/// Creates a banned embed
fn ban_embed(data: &PunishmentData, purged_message_second: &i64) -> EmbedBuilder {
    let mut embed = EmbedBuilder::default();
    let purged_messages = match purged_message_second {
        0 => "No messages deleted".to_owned(),
        3_600 => "Previous Hour".to_owned(),
        21_600 => "Previous 6 Hours".to_owned(),
        43_200 => "Previous 12 Hours".to_owned(),
        86_400 => "Previous 24 Hours".to_owned(),
        259_200 => "Previous 3 Days".to_owned(),
        604_800 => "Previous 7 Days".to_owned(),
        num => format!("Deleted {num} seconds worth of messages"),
    };

    embed.create_field(
        "User Details",
        format!(
            "{MEMBER_EMOJI} <@{0}>\n{MEMBER_EMOJI} <@{1}>\n",
            data.target.user_id(),
            data.author.user_id()
        ),
        true,
    );

    let mut stats = match data.dm_successful {
        Some(true) => format!("{PRIVATE_EMOJI} {purged_messages}\n{JOIN_EMOJI} User has been notified\n"),
        Some(false) => format!("{PRIVATE_EMOJI} {purged_messages}\n{LEAVE_EMOJI} User was not notified\n"),
        None => format!("{PRIVATE_EMOJI} {purged_messages}\n"),
    };

    match data.reason.contains('`') {
        true => {
            embed.description(format!("{}", data.reason));
        }
        false => {
            if !data.reason.is_empty() {
                stats.push_str(&format!("{TICKET_EMOJI} {}", data.reason));
            }
        }
    };

    embed.create_field("Reason & Punishment", stats, true);

    if let Ok(timestamp) = SystemTime::now().duration_since(UNIX_EPOCH) {
        if let Ok(timestamp) = Timestamp::from_secs(timestamp.as_secs() as i64) {
            embed.set_timestamp(timestamp);
        }
    };

    embed
        .author(|author| {
            author.icon_url(data.author.avatar_url()).name(format!(
                "{} has been BANNED by {}!",
                data.target.username(),
                data.author.username()
            ))
        })
        .colour(crate::COLOUR_DANGER)
        .footer(|footer| footer.icon_url(data.guild.icon_url()).text(&data.guild.twilight_guild.name))
        .thumbnail(|thumbnail| thumbnail.url(data.target.avatar_url()));
    embed
}

/// Creates a banned embed
fn unban_embed(data: &PunishmentData) -> EmbedBuilder {
    let mut embed = EmbedBuilder::default();

    embed.create_field(
        "User Details",
        format!(
            "{MEMBER_EMOJI} <@{0}>\n{MEMBER_EMOJI} <@{1}>\n",
            data.target.user_id(),
            data.author.user_id()
        ),
        true,
    );

    let mut stats = match data.dm_successful {
        Some(true) => format!("{JOIN_EMOJI} User has been notified\n"),
        Some(false) => format!("{LEAVE_EMOJI} User was not notified\n"),
        None => format!(""),
    };

    match data.reason.contains('`') {
        true => {
            embed.description(format!("{}", data.reason));
        }
        false => {
            if !data.reason.is_empty() {
                stats.push_str(&format!("{TICKET_EMOJI} {}", data.reason));
            }
        }
    };

    embed.create_field("Reason", stats, true);

    if let Ok(timestamp) = SystemTime::now().duration_since(UNIX_EPOCH) {
        if let Ok(timestamp) = Timestamp::from_secs(timestamp.as_secs() as i64) {
            embed.set_timestamp(timestamp);
        }
    };

    embed
        .author(|author| {
            author.icon_url(data.author.avatar_url()).name(format!(
                "{} has been UNBANNED by {}!",
                data.target.username(),
                data.author.username()
            ))
        })
        .colour(crate::COLOUR_SUCCESS)
        .footer(|footer| footer.icon_url(data.guild.icon_url()).text(&data.guild.twilight_guild.name))
        .thumbnail(|thumbnail| thumbnail.url(data.target.avatar_url()));
    embed
}

/// Creates a banned embed
fn kick_embed(data: &PunishmentData) -> EmbedBuilder {
    let mut embed = EmbedBuilder::default();

    embed.create_field(
        "User Details",
        format!(
            "{MEMBER_EMOJI} <@{0}>\n{MEMBER_EMOJI} <@{1}>\n",
            data.target.user_id(),
            data.author.user_id()
        ),
        true,
    );

    let mut stats = match data.dm_successful {
        Some(true) => format!("{JOIN_EMOJI} User has been notified\n"),
        Some(false) => format!("{LEAVE_EMOJI} User was not notified\n"),
        None => format!(""),
    };

    match data.reason.contains('`') {
        true => {
            embed.description(format!("{}", data.reason));
        }
        false => {
            if !data.reason.is_empty() {
                stats.push_str(&format!("{TICKET_EMOJI} {}", data.reason));
            }
        }
    };

    embed.create_field("Reason", stats, true);

    if let Ok(timestamp) = SystemTime::now().duration_since(UNIX_EPOCH) {
        if let Ok(timestamp) = Timestamp::from_secs(timestamp.as_secs() as i64) {
            embed.set_timestamp(timestamp);
        }
    };

    embed
        .author(|author| {
            author.icon_url(data.author.avatar_url()).name(format!(
                "{} has been KICKED by {}!",
                data.target.username(),
                data.author.username()
            ))
        })
        .colour(crate::COLOUR_DANGER)
        .footer(|footer| footer.icon_url(data.guild.icon_url()).text(&data.guild.twilight_guild.name))
        .thumbnail(|thumbnail| thumbnail.url(data.target.avatar_url()));
    embed
}

/// Creates a banned embed
fn warn_embed(data: &PunishmentData) -> EmbedBuilder {
    let mut embed = EmbedBuilder::default();

    embed.create_field(
        "User Details",
        format!(
            "{MEMBER_EMOJI} <@{0}>\n{MEMBER_EMOJI} <@{1}>\n",
            data.target.user_id(),
            data.author.user_id()
        ),
        true,
    );

    let mut stats = match data.dm_successful {
        Some(true) => format!("{JOIN_EMOJI} User has been notified\n"),
        Some(false) => format!("{LEAVE_EMOJI} User was not notified\n"),
        None => format!(""),
    };

    match data.reason.contains('`') {
        true => {
            embed.description(format!("{}", data.reason));
        }
        false => {
            if !data.reason.is_empty() {
                stats.push_str(&format!("{TICKET_EMOJI} {}", data.reason));
            }
        }
    };

    embed.create_field("Reason", stats, true);

    if let Ok(timestamp) = SystemTime::now().duration_since(UNIX_EPOCH) {
        if let Ok(timestamp) = Timestamp::from_secs(timestamp.as_secs() as i64) {
            embed.set_timestamp(timestamp);
        }
    };

    embed
        .author(|author| {
            author.icon_url(data.author.avatar_url()).name(format!(
                "{} has been WARNED by {}!",
                data.target.username(),
                data.author.username()
            ))
        })
        .colour(crate::COLOUR_DANGER)
        .footer(|footer| footer.icon_url(data.guild.icon_url()).text(&data.guild.twilight_guild.name))
        .thumbnail(|thumbnail| thumbnail.url(data.target.avatar_url()));
    embed
}
