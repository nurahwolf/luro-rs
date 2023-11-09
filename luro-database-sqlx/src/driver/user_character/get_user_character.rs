use luro_model::user::character::CharacterProfile;
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
                name: character.character_name,
                short_description: character.sfw_summary,
                icon: todo!(),
                nsfw_icon: todo!(),
                description: character.sfw_description,
                nsfw_description: character.nsfw_description,
                nsfw: todo!(),
                fetishes: todo!(),
                images: todo!(),
            }),
            None => None,
        })
    }
}
