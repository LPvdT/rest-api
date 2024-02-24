use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(super) struct CreateEntryData {
    pub(crate) title: String,
    pub(crate) date: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub(super) struct UpdateEntryData {
    pub(crate) title: String,
}
