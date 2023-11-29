use crate::call_to_action::CallToActions;
use crate::user::TweetUser;
use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TweetEntity {
    hashtags: Vec<TweetHashtag>,
    symbols: Vec<TweetSymbol>,
    pub user_mentions: Vec<TweetUser>,
    urls: Vec<TweetURL>,
    media: Option<Vec<TweetMedia>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
struct SizeMetadata {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    w: u128,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    h: u128,
    resize: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TweetMediaSizes {
    thumb: SizeMetadata,
    small: SizeMetadata,
    large: SizeMetadata,
    medium: SizeMetadata,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
struct VideoVariantMetadata {
    // TODO Deserialize optional string to optional number
    bitrate: Option<String>,
    content_type: String,
    url: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TweetVideoMetadata {
    aspect_ratio: (String, String),
    // TODO Deserialize optional string to optional number
    duration_millis: Option<String>,
    variants: Vec<VideoVariantMetadata>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
struct AdditionalMediaInfo {
    monetizable: bool,
    embeddable: Option<bool>,
    title: Option<String>,
    description: Option<String>,
    call_to_actions: Option<CallToActions>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TweetMedia {
    id: String,
    id_str: String,
    #[serde(rename = "type")]
    type_: String,
    url: String,
    display_url: String,
    media_url: String,
    media_url_https: String,
    sizes: TweetMediaSizes,
    expanded_url: String,
    indices: (String, String),
    source_status_id: Option<String>,
    source_status_id_str: Option<String>,
    source_user_id: Option<String>,
    source_user_id_str: Option<String>,
    video_info: Option<TweetVideoMetadata>,
    additional_media_info: Option<AdditionalMediaInfo>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
struct TweetSymbol {
    text: String,
    indices: (String, String),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TweetExtendedEntity {
    media: Vec<TweetMedia>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
struct TweetHashtag {
    text: String,
    indices: (String, String),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
struct TweetURL {
    url: String,
    expanded_url: String,
    display_url: String,
    indices: (String, String),
}
