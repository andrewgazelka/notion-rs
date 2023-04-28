use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BlockChildren {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_cursor: Option<String>,
    pub page_size: u32,
}

impl Default for BlockChildren {
    fn default() -> Self {
        Self {
            start_cursor: None,
            page_size: 100,
        }
    }
}
