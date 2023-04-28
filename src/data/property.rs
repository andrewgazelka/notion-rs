#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PropertyData {
    Checkbox(bool),
    CreatedBy(super::User),
    CreatedTime(super::DateTime),
    Date(Date),
    Email(String),
    Files(Vec<File>),
    Formula(Formula),
    LastEditedBy(super::User),
    LastEditedTime(super::DateTime),
    MultiSelect(serde_json::Value),
    Number(i64),
    People(Vec<super::User>),
    PhoneNumber(String),
    Relation(serde_json::Value),
    Rollup(serde_json::Value),
    RichText(RichText),
    Select(serde_json::Value),
    Status(Status),
    Title(Vec<super::TextBlock>),
    Url(String),
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct RichText(serde_json::Value);

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Property {
    pub id: String,
    #[serde(flatten)]
    pub data: PropertyData,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Formula {
    Boolean(bool),
    Date(super::Date),
    // TODO: is this the right date type
    Number(i64),
    String(String),
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct File {
    pub name: String,
    pub r#type: String,
    pub external_url: serde_json::Value,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Date {
    pub start: Option<String>,
    pub end: Option<String>,
    pub time_zone: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Status {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn deser_test() {
        let input = r#"
                    {
                "id": "title",
                "title": [
                {
                    "annotations": {
                    "bold": false,
                    "code": false,
                    "color": "default",
                    "italic": false,
                    "strikethrough": false,
                    "underline": false
                },
                    "href": null,
                    "plain_text": "Examples",
                    "text": {
                    "content": "Examples",
                    "link": null
                },
                    "type": "text"
                }
                ],
                "type": "title"
            }
        "#;

        let prop = serde_json::from_str::<super::Property>(input).unwrap();
        println!("{prop:#?}");
    }
}
