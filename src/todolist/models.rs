use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateEntryData {
    pub title: String,
    pub date: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateEntryData {
    pub title: String,
}
