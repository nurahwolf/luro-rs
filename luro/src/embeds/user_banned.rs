use std::fmt::Write as FmtWrite;

use crate::{
    builders::EmbedBuilder,
    models::{
        emoji::{JOIN, LEAVE, MAIL, MEMBER_EMOJI, PRIVATE_EMOJI, TICKET_EMOJI},
        MemberContext, User,
    },
};

pub enum PunishmentType {
    Banned,
}

pub struct Punishment<'a> {
    pub punishment_type: PunishmentType,
    pub moderator: &'a MemberContext,
    pub target: &'a User,
    pub reason: Option<&'a str>,
    pub purged_messages: i64,
    pub guild_name: &'a str,
    pub dm_success: Option<bool>,
}

impl<'a> Punishment<'a> {
    pub fn embed(&self) -> Result<EmbedBuilder, std::fmt::Error> {
        let mut embed = crate::builders::EmbedBuilder::default();
        let guild_name = self.guild_name;
        let author_name = self.moderator.username();
        let target_id = self.target.user_id();
        let purged_messages = match self.purged_messages {
            0 => "No messages deleted".to_owned(),
            3_600 => "Previous Hour".to_owned(),
            21_600 => "Previous 6 Hours".to_owned(),
            43_200 => "Previous 12 Hours".to_owned(),
            86_400 => "Previous 24 Hours".to_owned(),
            259_200 => "Previous 3 Days".to_owned(),
            604_800 => "Previous 7 Days".to_owned(),
            num => format!("Deleted {num} seconds worth of messages"),
        };

        let mut description = String::new();
        writeln!(description, "{MEMBER_EMOJI}<@{target_id}> | `{target_id}`")?;
        match self.dm_success {
            Some(false) => writeln!(
                description,
                "{PRIVATE_EMOJI}`{purged_messages}` | {MAIL}`Failed to notify user`{LEAVE}"
            )?,
            Some(true) => writeln!(
                description,
                "{PRIVATE_EMOJI}`{purged_messages}` | {MAIL}`User has been notified`{JOIN}"
            )?,
            None => writeln!(description, "{PRIVATE_EMOJI}`{purged_messages}`")?,
        }

        if let Some(reason) = self.reason {
            match reason.starts_with("```") {
                true => writeln!(description, "{TICKET_EMOJI}\n{reason}")?,
                false => writeln!(description, "{TICKET_EMOJI}`{reason}`")?,
            }
        }

        embed
            .colour(crate::COLOUR_DANGER)
            .thumbnail(|thumbnail| thumbnail.url(self.target.avatar_url()))
            .description(description)
            .author(|a| {
                a.icon_url(self.moderator.avatar_url())
                    .name(format!("BANNED by {author_name}! [{guild_name}]"))
            });

        Ok(embed)
    }
}
