use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    pub data: UserData
}

#[derive(Deserialize)]
pub struct UserData {
    pub id: String,                        // The user's Twitter identifier.
    pub name: String,                      // The user's display name.
    pub username: String,                  // The user's username / handle.
    pub created_at: DateTime<Utc>,         // The user's date of when they joined Twitter, in UTC.
    pub protected: bool,                   // The user's protected account status, e.g. whether or not tweets are private.
    pub location: Option<String>,          // The user's provided location, if available.
    pub description: String,               // The user's description / bio.
    pub verified: bool,                    // The user's verified status.
    pub profile_image_url: String,         // The user's profile image.
    pub public_metrics: UserPublicMetrics  // The user's publicly available metrics, such as followers / following.
}

#[derive(Deserialize)]
pub struct UserPublicMetrics {
    pub followers_count: u64, // The amount of people that follow the given user.
    pub following_count: u64, // The amount of people that the given user is following.
    pub tweet_count: u64      // The total amount of times the given user has Tweeted.
}

#[derive(Deserialize)]
pub struct UserTweets {
    pub data: Option<Vec<UserTweet>>
}

#[derive(Deserialize)]
pub struct UserTweet {
    pub text: String // the text
}
