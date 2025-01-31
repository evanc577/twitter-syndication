use serde::{Deserialize, Serialize};

#[allow(clippy::large_enum_variant)]
#[derive(Deserialize, Serialize, Clone, Hash, Debug)]
#[serde(tag = "__typename")]
pub(crate) enum TweetType {
    Tweet(Tweet),
    TweetTombstone,
}

#[derive(Deserialize, Serialize, Clone, Hash, Debug)]
pub struct Tweet {
    pub lang: String,
    pub conversation_count: u64,
    pub created_at: String,
    pub favorite_count: u64,
    pub id_str: String,
    pub text: String,
    #[serde(rename = "isEdited")]
    pub is_edited: bool,
    #[serde(rename = "isStaleEdit")]
    pub is_stale_edit: bool,

    pub user: User,
    #[serde(default)]
    pub photos: Vec<Photo>,
    pub video: Option<Video>,
}

#[derive(Deserialize, Serialize, Clone, Hash, Debug)]
pub struct User {
    pub id_str: String,
    pub name: String,
    pub profile_image_url_https: String,
    pub screen_name: String,
    pub verified: bool,
    pub is_blue_verified: bool,
}

#[derive(Deserialize, Serialize, Clone, Hash, Debug)]
pub struct Photo {
    pub url: String,
    pub width: u64,
    pub height: u64,
}

#[derive(Deserialize, Serialize, Clone, Hash, Debug)]
pub struct Video {
    #[serde(rename = "aspectRatio")]
    pub aspect_ratio: [u64; 2],
    #[serde(rename = "durationMs")]
    pub duration_ms: u64,
    pub poster: String,
    pub variants: Vec<VideoVariant>,
}

#[derive(Deserialize, Serialize, Clone, Hash, Debug)]
pub struct VideoVariant {
    pub r#type: String,
    pub src: String,
}
