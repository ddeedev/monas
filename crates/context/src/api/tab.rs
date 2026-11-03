use crate::{Tab, TabData};
use http::Method;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use std::collections::HashMap;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequestData {
    pub url: String,
    pub tab_number: Option<u8>,
    // for api request favicon will ba base on its method
    pub favicon: String,
    #[serde_as(as = "DisplayFromStr")]
    pub method: Method,
    pub params: HashMap<String, String>,
    pub body: String,
    pub authorization: String,
    pub headers: HashMap<String, String>,
}

impl ApiRequestData {
    pub fn new(
        url: String,
        method: Method,
        params: HashMap<String, String>,
        body: String,
        auth: String,
        headers: HashMap<String, String>,
    ) -> Self {
        let mut favicon: String = String::new();
        match method {
            Method::POST => {
                favicon.push_str("RED");
            }
            Method::GET => {
                favicon.push_str("GREEN");
            }
            Method::PUT => {
                favicon.push_str("CYAN");
            }
            Method::DELETE => {
                favicon.push_str("DARK");
            }
            Method::PATCH => {
                favicon.push_str("YELLOW");
            }
            _ => favicon.push_str("WHITE"),
        };
        Self {
            url,
            authorization: auth,
            body,
            headers,
            params,
            favicon,
            method,
            tab_number: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ApiRequestParams {
    pub url: String,
    pub method: Method,
    pub params: HashMap<String, String>,
    pub body: String,
    pub auth: String,
    pub headers: HashMap<String, String>,
}

impl Tab for ApiRequestData {
    type Params = ApiRequestParams;

    fn new_tab(params: Self::Params) -> Self {
        ApiRequestData::new(
            params.url,
            params.method,
            params.params,
            params.body,
            params.auth,
            params.headers,
        )
    }
}

impl From<ApiRequestData> for TabData {
    fn from(value: ApiRequestData) -> Self {
        TabData::ApiRequest(value)
    }
}
