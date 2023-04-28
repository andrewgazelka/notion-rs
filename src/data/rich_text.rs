use serde::{Deserialize, Serialize};

use crate::data::Annotations;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Link {
    url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RichTextData {
    Text {
        content: String,
        link: Option<Link>,
    },
    /// TODO: finish
    Mention {
        r#type: String,
    },
    Equation {
        expression: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RichText {
    pub r#type: String,

    #[serde(flatten)]
    pub data: RichTextData,
    pub annotations: Annotations,
    pub plain_text: String,
    pub href: Option<String>,
}

impl RichText {
    pub fn to_markdown(&self) -> String {
        use RichTextData::{Equation, Mention, Text};
        match &self.data {
            Text { content, link } => {
                let mut res = content.clone();
                if let Some(link) = link {
                    res = format!("[{}]({})", res, link.url);
                }
                res
            }
            Equation { expression } => format!("$${expression}$$"),
            #[allow(clippy::uninlined_format_args)]
            Mention { r#type } => format!("@{}", r#type), // TODO: include user name
        }
    }
}
