use futures_util::TryStreamExt;
use luro_model::user::character::CharacterProfile;
use twilight_model::id::{marker::UserMarker, Id};

impl crate::SQLxDriver {
    pub async fn get_user_characters(&self, user_id: Id<UserMarker>) -> Result<Vec<CharacterProfile>, sqlx::Error> {
        let mut characters = vec![];
        let mut query = sqlx::query!(
            "
            SELECT * FROM user_characters WHERE (user_id = $1)
            ",
            user_id.get() as i64
        )
        .fetch(&self.pool);

        while let Ok(Some(character)) = query.try_next().await {
            characters.push(CharacterProfile {
                name: character.character_name,
                short_description: character.sfw_summary,
                icon: todo!(),
                nsfw_icon: todo!(),
                description: character.sfw_description,
                nsfw_description: character.nsfw_description,
                nsfw: todo!(),
                fetishes: todo!(),
                images: todo!(),
            })
        }

        Ok(characters)
    }
}
