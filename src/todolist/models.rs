use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(super) struct CreateEntryData {
    pub title: String,
    pub date: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub(super) struct UpdateEntryData {
    pub title: String,
}
