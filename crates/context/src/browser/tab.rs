use crate::api::tab::ApiRequestData;
use http::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TabData {
    Browser(BrowserData),
    ApiRequest(ApiRequestData),
}

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

impl TabData {
    pub fn new_browser_tab(url: String, favicon: String) -> Self {
        Self::Browser(BrowserData::new(url, favicon))
    }

    pub fn new_api_tab(
        url: String,
        method: Method,
        params: HashMap<String, String>,
        body: String,
        auth: String,
        headers: HashMap<String, String>,
    ) -> Self {
        Self::ApiRequest(ApiRequestData::new(
            url, method, params, body, auth, headers,
        ))
    }
}
