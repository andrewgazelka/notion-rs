//! <https://developers.notion.com/docs/working-with-page-content>
//! <https://developers.notion.com/reference/get-block-children>
//! <https://rustwasm.github.io/docs/wasm-bindgen/>

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

mod block;
mod color;
pub mod parent_object;
mod property;
mod rich_text;

pub use block::{Block, BlockData};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SortTimestamp {
    LastEditedTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sort {
    pub direction: SortDirection,
    pub timestamp: SortTimestamp,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter<'a> {
    /// The value of the property to filter the results by. Possible values for object type include
    /// page or database. Limitation: Currently the only filter allowed is object which will filter
    /// by type of object (either page or database)
    pub value: &'a str,
    /// The name of the property to filter by. Currently the only property you can filter by is the
    /// object type. Possible values include object. Limitation: Currently the only filter allowed
    /// is object which will filter by type of object (either page or database)
    pub property: &'a str,
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
pub struct SearchRequest<'a> {
    /// The text that the API compares page and database titles against.
    pub query: Option<&'a str>,
    pub sort: Option<Sort>,

    pub filter: Option<Filter<'a>>,
    pub start_cursor: Option<&'a str>,

    /// The maximum number of results to return. Default: 100. Maximum: 100.
    pub page_size: Option<u32>,
}

impl Default for SearchRequest<'_> {
    fn default() -> Self {
        Self {
            query: None,
            sort: None,
            filter: None,
            start_cursor: None,
            page_size: Some(100),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "object")]
#[serde(rename_all = "snake_case")]
pub enum Object {
    User(User),
    List(List),
    Page(Page),
    Block(Box<Block>),
}

// #[derive(Serialize, Deserialize, Debug)]
// struct BlockObject {
//     archived: bool,
//     created_by: User,
//     created_time: DateTime,
//     has_children: bool,
//
//     #[serde(flatten)]
//     block: Block,
// }
//
/// `2023-03-08T18:25:00.000Z`
type DateTime = String;

/// `2023-03-08`
type Date = String;

/// [Reference](https://developers.notion.com/reference/page)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page {
    /// `be633bf1-dfa0-436d-b259-571129a590e5`
    pub id: String,
    pub created_time: DateTime,
    pub last_edited_time: DateTime,
    pub created_by: User,
    pub last_edited_by: User,
    pub cover: Option<serde_json::Value>,
    pub icon: Option<serde_json::Value>,
    pub parent: Option<parent_object::ParentObject>,
    pub archived: bool,
    pub properties: HashMap<String, property::Property>,
    pub url: String,
}

impl Page {
    #[must_use]
    pub fn property(&self, name: &str) -> Option<&property::PropertyData> {
        let data = &self.properties.get(name)?.data;
        Some(data)
    }

    #[must_use]
    pub fn title(&self) -> Option<&TextBlock> {
        let property::PropertyData::Title(title) = self.property("title")? else {
            return None;
        };
        title.get(0)
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum PropertyType {
    Checkbox,
    CreatedBy,
    CreatedTime,
    Date,
    Email,
    Files,
    Formula,
    LastEditedBy,
    LastEditedTime,
    MultiSelect,
    Number,
    People,
    PhoneNumber,
    Relation,
    Rollup,
    RichText,
    Select,
    Status,
    Title,
    Url,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct List {
    pub results: Vec<Object>,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub r#type: Option<String>,
    pub person: Option<Person>,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    pub email: String,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone, Default)]
pub struct TextData {
    pub content: Option<String>,
    pub link: Option<String>,
}

// /// <https://developers.notion.com/reference/block>
// #[derive(Eq, PartialEq, Serialize, Deserialize, Debug)]
// #[serde(tag = "type")]
// #[serde(rename_all = "snake_case")]
// pub enum BlockData {
//     Paragraph(ParagraphBlock),
//     Text(TextBlock),
//     PageId {
//         page_id: String,
//     },
//     Workspace {
//         workspace: bool,
//     },
//     ChildPage {
//         child_page: String,
//     },
//     // Heading1(Heading1Block),
//     // Heading2(Heading2Block),
//     // Heading3(Heading3Block),
//     // BulletedListItem(BulletedListItemBlock),
//     // NumberedListItem(NumberedListItemBlock),
//     // ToDo(ToDoBlock),
//     // Toggle(ToggleBlock),
//     // ChildPage(ChildPageBlock),
//     Unsupported,
// }

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone, Hash, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct Annotations {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: String,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, Clone, Default)]
pub struct TextBlock {
    pub text: TextData,
    pub annotations: Annotations,
    pub plain_text: String,
    pub href: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    pub object: String,
    pub results: Vec<Page>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
    pub r#type: String,
    pub page_or_database: serde_json::Value,
}

#[cfg(test)]
mod tests {
    // use crate::data::{Annotations, Block, Object, Page, TextBlock, TextData};

    // #[test]
    // fn test_deserialize() {
    //     let grocery = serde_json::json! {{
    //         "type": "text",
    //         "text": {
    //             "content": "Grocery List",
    //             "link": null
    //         },
    //         "annotations": {
    //             "bold": false,
    //             "italic": false,
    //             "strikethrough": false,
    //             "underline": false,
    //             "code": false,
    //             "color": "default"
    //         },
    //         "plain_text": "Grocery List",
    //         "href": null
    //     }};
    //
    //     let block: Block = serde_json::from_value(grocery).unwrap();
    //
    //     let expected = Block::Text(TextBlock {
    //         text: TextData {
    //             content: Some("Grocery List".to_string()),
    //             link: None,
    //         },
    //         annotations: Annotations {
    //             bold: false,
    //             italic: false,
    //             strikethrough: false,
    //             underline: false,
    //             code: false,
    //             color: "default".to_string(),
    //         },
    //         plain_text: "Grocery List".to_string(),
    //         href: None,
    //     });
    //
    //     assert_eq!(block, expected);
    // }

    // #[test]
    // fn test_deserialize_page() {
    //     let input = r#"
    //     {
    //         "archived": false,
    //         "cover": null,
    //         "created_by": {
    //             "id": "077a0175-ad9e-4ea0-bab1-6c2737500371",
    //             "object": "user"
    //         },
    //         "created_time": "2023-03-13T03:12:00.000Z",
    //         "icon": null,
    //         "id": "69202e6a-a005-45cb-ae3d-1b2f8a2a022b",
    //         "last_edited_by": {
    //             "id": "89a442ee-c522-422d-8552-879d3abca760",
    //             "object": "user"
    //         },
    //         "last_edited_time": "2023-03-13T21:10:00.000Z",
    //         "object": "page",
    //         "parent": {
    //             "type": "workspace",
    //             "workspace": true
    //         },
    //         "properties": {
    //         "title": {
    //             "id": "title",
    //             "title": [
    //             {
    //                 "annotations": {
    //                 "bold": false,
    //                 "code": false,
    //                 "color": "default",
    //                 "italic": false,
    //                 "strikethrough": false,
    //                 "underline": false
    //             },
    //                 "href": null,
    //                 "plain_text": "Examples",
    //                 "text": {
    //                 "content": "Examples",
    //                 "link": null
    //             },
    //                 "type": "text"
    //             }
    //             ],
    //             "type": "title"
    //         }
    //     },
    //         "url": "https://www.notion.so/Examples-69202e6aa00545cbae3d1b2f8a2a022b"
    //     }
    // "#;
    //
    //     let output = serde_json::from_str::<Page>(input).unwrap();
    //     println!("{:#?}", output)
    // }

    // #[test]
    // fn block_deser(){
    //     let json = r#"{
    //       "archived": false,
    //       "child_page": {
    //         "title": "Meeting Notes 1"
    //       },
    //       "created_by": {
    //         "id": "077a0175-ad9e-4ea0-bab1-6c2737500371",
    //         "object": "user"
    //       },
    //       "created_time": "2023-03-13T03:32:00.000Z",
    //       "has_children": true,
    //       "id": "c2d57097-582a-4ddc-b2e7-9c7f64c21923",
    //       "last_edited_by": {
    //         "id": "077a0175-ad9e-4ea0-bab1-6c2737500371",
    //         "object": "user"
    //       },
    //       "last_edited_time": "2023-03-13T03:41:00.000Z",
    //       "object": "block",
    //       "parent": {
    //         "page_id": "1e06feba-b7de-472a-a39b-7a3b0f1c67a2",
    //         "type": "page_id"
    //       },
    //       "type": "child_page"
    //     }
    //     "#;
    //
    //     let v: Object = serde_json::from_str(json).unwrap();
    //     println!("{:#?}", v);
    // }
}
