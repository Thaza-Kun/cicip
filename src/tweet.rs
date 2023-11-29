use crate::entities::TweetEntity;
use crate::entities::TweetExtendedEntity;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TweetItem {
    #[serde(deserialize_with = "my_date_format::deserialize")]
    created_at: DateTime<Utc>,
    id: String,
    id_str: String,
    pub full_text: String,
    favorited: bool,
    retweeted: bool,
    display_text_range: (String, String),
    source: String,
    entities: TweetEntity,
    // TODO deserialize with
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub favorite_count: u128,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub retweet_count: u128,
    in_reply_to_screen_name: Option<String>,
    in_reply_to_status_id: Option<String>,
    in_reply_to_status_id_str: Option<String>,
    in_reply_to_user_id: Option<String>,
    in_reply_to_user_id_str: Option<String>,
    truncated: bool,
    lang: String,
    contributors: Option<Vec<String>>,
    possibly_sensitive: Option<bool>,
    extended_entities: Option<TweetExtendedEntity>,
    withheld_copyright: Option<bool>,
    withheld_in_countries: Option<Vec<String>>,
}
impl Display for TweetItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            r#"[TweetItem] https://twitter.com/Thaza_Kun/status/{}/
{}

---
[FAV]: {}  [RT]: {}
"#,
            self.id, self.full_text, self.favorite_count, self.retweet_count
        )
    }
}

impl TweetItem {

    pub fn from_json(json_str: String) -> Vec<TweetItem> {
        let tweets = serde_json::from_str::<Vec<Tweets>>(&json_str).unwrap();
        tweets.iter().map(|a| a.clone().tweet).collect::<Vec<TweetItem>>()
    }
    pub fn is_a_retweet(&self) -> bool {
        self.full_text.contains("RT")
    }

    pub fn is_mentioning(&self) -> bool {
        (self.entities.user_mentions.len() != 0) || (self.in_reply_to_status_id == None)
    }

    pub fn get_comments<'a>(&'a self, others: &'a Vec<TweetItem>) -> Vec<&TweetItem> {
        others
            .iter()
            .filter(|a| a.in_reply_to_status_id.clone().unwrap_or("".into()) == self.id)
            .collect()
    }

    pub fn chain_heads<'a>(&'a self, others: &'a Vec<TweetItem>) -> Vec<&TweetItem> {
        let mut chain = Vec::<&TweetItem>::new();
        let mut head = others
            .iter()
            .filter(|a| a.id == self.clone().in_reply_to_status_id.unwrap_or("".into()))
            .collect::<Vec<&TweetItem>>();
        loop {
            match head.first() {
                None => break,
                Some(T) => {
                    chain.push(*T);
                    head = others
                        .iter()
                        .filter(|a| a.id == T.in_reply_to_status_id.clone().unwrap_or("".into()))
                        .collect::<Vec<&TweetItem>>();
                }
            }
        }
        chain
    }
}

mod my_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    // Mon Jan 23 15:25:23 +0000 2017
    const FORMAT: &'static str = "%a %b %d %H:%M:%S +0000 %Y";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
struct Tweets {
    tweet: TweetItem,
}
