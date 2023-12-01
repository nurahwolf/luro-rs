use tokio_postgres::Client;
use twilight_model::util::ImageHash;

mod conversions;
mod cornucopia;
mod driver;

#[derive(Debug)]
pub struct Driver {
    pub client: Client,
}

impl Driver {
    pub async fn new() -> Result<Self, tokio_postgres::Error> {
        let (client, connection) = tokio_postgres::connect("host=localhost dbname=luro", tokio_postgres::NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Self { client })
    }
}

pub fn handle_img(img: Option<&str>) -> Option<ImageHash> {
    img.and_then(|x| match ImageHash::parse(x.as_bytes()) {
        Ok(img) => Some(img),
        Err(why) => {
            tracing::error!("handle_img - Failed to parse with the following error: {why:?}");
            None
        }
    })
}
