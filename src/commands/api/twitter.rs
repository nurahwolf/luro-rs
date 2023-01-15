use luro_utilities::{format_int, guild_accent_colour};

use crate::{
    structs::twitter::{User, UserTweets},
    Context, Error
};

/// Display information on a twitter user
#[poise::command(prefix_command, slash_command, category = "API")]
pub async fn twitter(
    ctx: Context<'_>,
    #[rest]
    #[description = "The user to get"]
    user: String
) -> Result<(), Error> {
    let user_fields = [(
        "user.fields",
        "created_at,protected,location,public_metrics,description,verified,profile_image_url"
    )];
    let tweet_fields = [("max_results", "5"), ("exclude", "retweets,replies")];
    let client = reqwest::Client::builder().user_agent("Luro/1.0 (nurah@wolfo.tech)").build()?;
    let mut endpoint = format!("https://api.twitter.com/2/users/by/username/{user}");
    let mut request = client
        .get(&endpoint)
        .bearer_auth(ctx.data().secrets.twitter_api.clone().unwrap())
        .query(&user_fields)
        .send()
        .await?;

    let user = request.json::<User>().await?.data;
    let id = &user.id;
    let name = &user.name;
    let handle = user.username;
    let joined = user.created_at.format("%B %-e, %Y").to_string();
    let protected = if user.protected { "Yes" } else { "No" }.to_string();
    let location = user.location;
    let url = format!("https://twitter.com/{handle}");
    let description = &user.description;
    let avatar = &user.profile_image_url.replace("normal", "400x400").to_string();
    let following = format_int(user.public_metrics.following_count);
    let followers = format_int(user.public_metrics.followers_count);
    let tweets = format_int(user.public_metrics.tweet_count);

    endpoint = format!("https://api.twitter.com/2/users/{id}/tweets");
    request = client
        .get(&endpoint)
        .bearer_auth(ctx.data().secrets.twitter_api.clone().unwrap())
        .query(&tweet_fields)
        .send()
        .await?;

    let tweets_response: UserTweets = request.json().await?;
    let latest_tweet = match tweets_response.data {
        Some(tweets) => tweets.first().unwrap().text.clone(),
        None => "Tweet not available.".to_string()
    };

    let accent_colour = ctx.data().config.read().await.accent_colour;
    ctx.send(|builder| {
        builder.embed(|embed| {
            embed
                .title(format!(
                    "{name}{verified}",
                    verified = if user.verified { " \\✔️" } else { "" }
                ))
                .url(url)
                .thumbnail(avatar)
                .color(guild_accent_colour(accent_colour, ctx.guild()))
                .description(description)
                .fields(vec![
                    ("Username", handle, true),
                    ("Join Date", joined, true),
                    ("Protected", protected, true),
                    (
                        "Location",
                        if location.is_some() {
                            location.unwrap()
                        } else {
                            "None".to_string()
                        },
                        true
                    ),
                    ("Following", following, true),
                    ("Followers", followers, true),
                    ("Tweets", tweets, true),
                    ("Latest Tweet", latest_tweet, false),
                ])
                .footer(|footer| footer.text(format!("User ID: {id} | Powered by Twitter.")))
        })
    })
    .await?;

    Ok(())
}
