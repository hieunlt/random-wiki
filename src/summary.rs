use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Summary {
    pub description: String,
    pub title: String,
    pub extract: String,
    pub content_urls: HashMap<String, ContentUrl>,
}
#[derive(Debug, Deserialize)]
pub struct ContentUrl {
    pub page: String,
}
