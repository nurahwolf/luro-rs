mod character_fetish;
mod character_fetish_category;
mod character_fetish_list;
mod character_image;
mod character_profile;

pub use character_fetish::CharacterFetish;
pub use character_fetish_category::{CharacterFetishCategory, FetishCategory};
pub use character_fetish_list::FetishList;
pub use character_image::CharacterImage;
pub use character_profile::CharacterProfile;
use twilight_model::id::{marker::UserMarker, Id};

use crate::database::{Database, Error};

#[derive(Clone, Debug, Default, serde::Deserialize, PartialEq, serde::Serialize)]
pub struct Fetish {
    #[serde(default)]
    pub category: FetishCategory,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub list: FetishList,
}

#[derive(Debug)]
pub struct Character<'a> {
    pub db: &'a Database,
    pub name: String,
    pub nsfw_description: Option<String>,
    pub nsfw_icons: Option<Vec<String>>,
    pub nsfw_summary: Option<String>,
    pub prefix: Option<String>,
    pub sfw_description: String,
    pub sfw_icons: Option<Vec<String>>,
    pub sfw_summary: String,
    pub user_id: Id<UserMarker>,
}

impl<'a> Character<'a> {
    pub async fn new_image(&self, img: &CharacterImage) -> Result<CharacterImage, Error> {
        self.db.create_character_image(img).await
    }

    pub async fn update_image(&self, img: &CharacterImage) -> Result<CharacterImage, Error> {
        self.db.update_character_image(img).await
    }

    pub async fn update_fetish(&self, fetish: CharacterFetish) -> Result<CharacterFetish, Error> {
        self.db.update_character_fetish(fetish.into()).await.map(|x| x.into())
    }

    pub async fn fetch_images(&self) -> Result<Vec<CharacterImage>, Error> {
        self.db.fetch_character_images(self.user_id, &self.name).await
    }

    pub async fn fetch_image(&self, img_id: i64) -> Result<Option<CharacterImage>, Error> {
        Ok(self.db.fetch_character_image(self.user_id, &self.name, img_id).await?)
    }

    pub async fn fetch_fetishes(&self) -> Result<Vec<CharacterFetish>, Error> {
        let character = self.clone().into();
        Ok(self
            .db
            .get_character_fetishes(&character)
            .await?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    pub async fn fetch_fetish(&self, fetish_id: i64) -> Result<Option<LuroCharacterFetish>, Error> {
        let character = self.clone().into();
        Ok(self.db.get_character_fetish(&character, fetish_id).await?.map(|x| x.into()))
    }
}
