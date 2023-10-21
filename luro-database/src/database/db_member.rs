use anyhow::anyhow;
use twilight_model::{
    gateway::payload::incoming::{MemberAdd, MemberChunk, MemberRemove, MemberUpdate},
    guild::{Member, PartialMember},
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};

use crate::{DbMember, DbMemberType, LuroDatabase, LuroMember};

impl From<(Id<GuildMarker>, Member)> for DbMemberType {
    fn from(data: (Id<GuildMarker>, Member)) -> Self {
        Self::Member(data.0, data.1)
    }
}

impl From<(Id<GuildMarker>, PartialMember)> for DbMemberType {
    fn from(data: (Id<GuildMarker>, PartialMember)) -> Self {
        Self::PartialMember(data.0, data.1)
    }
}

impl From<Box<MemberAdd>> for DbMemberType {
    fn from(data: Box<MemberAdd>) -> Self {
        Self::MemberAdd(data)
    }
}

impl From<MemberChunk> for DbMemberType {
    fn from(data: MemberChunk) -> Self {
        Self::MemberChunk(data)
    }
}

impl From<MemberRemove> for DbMemberType {
    fn from(data: MemberRemove) -> Self {
        Self::MemberRemove(data)
    }
}

impl From<Box<MemberUpdate>> for DbMemberType {
    fn from(data: Box<MemberUpdate>) -> Self {
        Self::MemberUpdate(data)
    }
}

impl From<LuroMember> for DbMemberType {
    fn from(data: LuroMember) -> Self {
        Self::LuroMember(data)
    }
}

impl DbMember {
    pub fn user_id(&self) -> Id<UserMarker> {
        Id::new(self.user_id as u64)
    }

    /// Return a string that is a Discord CDN link to the user's avatar
    pub async fn avatar(&self, db: &LuroDatabase) -> anyhow::Result<String> {
        let guild_id = self.guild_id;
        let user_id = self.user_id;

        // Return the member's avatar if present
        if let Some(avatar) = self.avatar.map(|x| x.0) {
            return Ok(match avatar.is_animated() {
                true => format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{avatar}.gif?size=2048"),
                false => format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{avatar}.png?size=2048"),
            });
        }

        // Oterwise return their user avatar
        match db.get_user(user_id).await? {
            Some(user) => Ok(user.avatar()),
            None => Err(anyhow!("No member avatar and the user does not exist in the DB")),
        }
    }

    /// Return a string that is a link to the user's banner, or [None] if they don't have one
    pub async fn banner(&self, db: &LuroDatabase) -> anyhow::Result<Option<String>> {
        // Return user banner, Twilight currently can't get banners from guilds
        match db.get_user(self.user_id).await? {
            Some(user) => Ok(user.banner()),
            None => Err(anyhow!("No member banner and the user does not exist in the DB")),
        }
    }
}
