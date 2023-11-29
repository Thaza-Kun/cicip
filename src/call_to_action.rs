use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
struct UrlCTA {
    url: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CallToActions {
    visit_site: Option<UrlCTA>,
    watch_now: Option<UrlCTA>,
}
