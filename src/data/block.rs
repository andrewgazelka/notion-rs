use crate::data::{color::Color, rich_text::RichText, Object};

/// <https://developers.notion.com/reference/block>
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Block {
    pub id: String,
    pub parent: Option<serde_json::Value>,

    /// TODO: this links to `data`. Is there anyway we can add
    /// `#[serde(rename = "type")]` to the `BlockData` enum?
    ///
    /// The issue is the type is used twice.
    pub r#type: String,

    #[serde(flatten)]
    pub data: BlockData,

    pub created_time: String,
    pub created_by: serde_json::Value,

    pub last_edited_time: String,
    pub last_edited_by: serde_json::Value,
    pub archived: bool,
    pub has_children: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Heading {
    pub rich_text: Vec<RichText>,
    pub color: Color,
    pub is_toggleable: bool,
}

/// <https://developers.notion.com/reference/block>
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BlockData {
    Bookmark {
        caption: Vec<RichText>,
        url: String,
    },
    BreadCrumb,
    BulletedListItem {
        rich_text: Vec<RichText>,
        color: Color,
    },
    Callout {
        rich_text: RichText,
        icon: Object,
        color: Color,
    },
    ChildDatabase {
        title: String,
    },
    ChildPage {
        title: String,
    },
    Column,
    ColumnList,
    Divider,
    Embed {
        url: String,
    },
    Equation {
        expression: String,
    },
    File {
        caption: Vec<RichText>,
        r#type: String,
        /// file object
        file: Object,
    },

    #[serde(rename = "heading_1")]
    Heading1(Heading),
    #[serde(rename = "heading_2")]
    Heading2(Heading),
    #[serde(rename = "heading_3")]
    Heading3(Heading),
    /// TODO
    Image,
    LinkPreview {
        url: String,
    },
    LinkToPage {
        url: String,
    },
    NumberedListItem {
        rich_text: Vec<RichText>,
        color: Color,
        #[serde(default)]
        children: Vec<Block>,
    },
    Paragraph {
        rich_text: Vec<RichText>,
        color: Color,
        #[serde(default)]
        children: Vec<Block>,
    },
    Pdf {
        caption: Vec<RichText>,
        r#type: String,
        /// file object
        file: Object,
    },
    Quote,
    SyncedBlock,
    Table,
    TableOfContents,
    TableRow,
    Template,
    ToDo,
    Toggle,
    Unsupported,
    Video,
}

impl BlockData {
    #[must_use]
    pub const fn heading(&self) -> Option<&Heading> {
        match self {
            Self::Heading1(heading) | Self::Heading2(heading) | Self::Heading3(heading) => {
                Some(heading)
            }
            _ => None,
        }
    }

    pub fn heading_mut(&mut self) -> Option<&mut Heading> {
        match self {
            Self::Heading1(heading) | Self::Heading2(heading) | Self::Heading3(heading) => {
                Some(heading)
            }
            _ => None,
        }
    }

    #[must_use]
    pub fn into_heading(self) -> Option<Heading> {
        match self {
            Self::Heading1(heading) | Self::Heading2(heading) | Self::Heading3(heading) => {
                Some(heading)
            }
            _ => None,
        }
    }
}
