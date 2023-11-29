use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TweetUser {
    name: String,
    screen_name: String,
    indices: (String, String),
    id_str: String,
    id: String,
}
