use crate::BrowserData;
use crate::api::tab::ApiRequestData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TabData {
    Browser(BrowserData),
    ApiRequest(ApiRequestData),
}

/// implement by every concrete tab type
pub trait Tab: Sized + Into<TabData> {
    /// whatever parameter of that tab type need
    type Params;

    fn new_tab(params: Self::Params) -> Self;

    // build and warp to enum
    fn create(params: Self::Params) -> TabData {
        Self::new_tab(params).into()
    }
}

impl TabData {
    // generic entry point
    pub fn new<T: Tab>(params: T::Params) -> Self {
        T::create(params)
    }
}
