use luro_model::types::CharacterProfile;
use twilight_model::id::{marker::UserMarker, Id};

impl crate::SQLxDriver {
    pub async fn get_user_character(&self, user_id: Id<UserMarker>, name: &str) -> Result<Option<CharacterProfile>, sqlx::Error> {
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
                sfw_description: character.sfw_description,
                sfw_summary: character.sfw_summary,
                sfw_icons: character.sfw_icons.unwrap_or_default(),
                nsfw_description: character.nsfw_description,
                nsfw_summary: character.nsfw_summary,
                nsfw_icons: character.nsfw_icons.unwrap_or_default(),
            }),
            None => None,
        })
    }
}
