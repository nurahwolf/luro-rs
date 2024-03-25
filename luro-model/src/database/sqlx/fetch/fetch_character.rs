use twilight_model::id::{marker::UserMarker, Id};

use crate::character::CharacterProfile;

impl crate::database::sqlx::Database {
    pub async fn fetch_character(&self, user_id: Id<UserMarker>, name: &str) -> Result<Option<CharacterProfile>, sqlx::Error> {
        let character = sqlx::query!(
            "
                SELECT * FROM user_characters WHERE (user_id = $1 and character_name = $2)
            ",
            user_id.get() as i64,
            name
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(match character {
            Some(character) => Some(CharacterProfile {
                prefix: character.prefix,
                name: character.character_name,
                nickname: character.nickname,
                colour: character.colour.map(|x| x as u32),
                sfw_description: character.sfw_description,
                sfw_summary: character.sfw_summary,
                sfw_icon: character.sfw_icon,
                nsfw_description: character.nsfw_description,
                nsfw_summary: character.nsfw_summary,
                nsfw_icon: character.nsfw_icon,
                user_id,
            }),
            None => None,
        })
    }
}
