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
