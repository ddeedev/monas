use crate::{TabData, tabs::Tab};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserData {
    pub url: String,
    pub tab_number: Option<u8>,
    pub favicon: String,
}

impl BrowserData {
    pub fn new(url: String, favicon: String) -> Self {
        Self {
            url,
            favicon,
            tab_number: None,
        }
    }

    pub fn favicon_for(page_url: &str) -> String {
        // TODO:: validate when move to utils func
        // validate
        // url::Url::parse(page_url).ok().and_then(|u| u.host_str().map(str::to_owned))
        let host = page_url
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .split("/")
            .next()
            .unwrap_or_default();

        format!("https://www.google.com/s2/favicons?domain={host}&sz=64")
    }
}

#[derive(Debug, Clone, Default)]
pub struct BrowserParams {
    pub url: String,
    pub favicon: String,
}

impl Tab for BrowserData {
    type Params = BrowserParams;

    fn new_tab(p: BrowserParams) -> Self {
        Self {
            url: p.url,
            favicon: p.favicon,
            tab_number: None,
        }
    }
}

impl From<BrowserData> for TabData {
    fn from(value: BrowserData) -> Self {
        TabData::Browser(value)
    }
}
