//! - [Page tutorial](https://developers.notion.com/docs/working-with-page-content)
//!     - Covers content vs properties
use std::format as f;

use anyhow::{bail, Result};
use iter_tools::Itertools;
use reqwest::{Method, RequestBuilder};
use tracing::instrument;

use crate::data::SearchResponse;
pub use crate::utils::CachedClient;

pub mod data;
pub mod query;
mod utils;

fn default<T: Default>() -> T {
    T::default()
}

/// - [Getting Started](https://developers.notion.com/docs/getting-started)
///   - [Create an Integration](https://developers.notion.com/docs/create-a-notion-integration)
#[derive(Clone)]
pub struct Client {
    req: reqwest::Client,
    integration_token: String,
}

impl Client {
    /// Create a new client with the given integration token.
    pub fn new(integration_token: impl Into<String>) -> Self {
        Self {
            req: reqwest::Client::new(),
            integration_token: integration_token.into(),
        }
    }

    fn request(&self, method: Method, url: &str) -> RequestBuilder {
        self.req
            .request(method, f!("https://api.notion.com/v1/{url}"))
            .header("Notion-Version", "2022-06-28")
            .header("Content-Type", "application/json")
            .bearer_auth(&self.integration_token)
    }

    /// # Errors
    /// - If the request fails.
    #[instrument(skip(self))]
    pub async fn list_users(&self) -> Result<Vec<data::User>> {
        let response = self
            .request(Method::GET, "users")
            .send()
            .await?
            .json()
            .await?;

        let data::Object::List(data::List { results }) = response else {
            bail!("Result {response:?} is not a list")
        };

        let res: Vec<_> = results
            .into_iter()
            .map(|result| {
                let data::Object::User(user) = result else {
                    bail!("Result {result:?} is not a user")
                };
                Ok(user)
            })
            .try_collect()?;

        Ok(res)
    }

    /// # Errors
    /// - If the request fails.
    #[instrument(skip(self), fields(id = %block_id))]
    pub async fn block(&self, block_id: &str) -> Result<data::Block> {
        let response = self
            .request(Method::GET, &f!("blocks/{block_id}"))
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    /// # Errors
    /// - If the request fails.
    /// - If the response is not a list.
    /// - If the response is not a block.
    #[instrument(skip(self))]
    pub async fn block_children(
        &self,
        block_id: &str,
        query: query::BlockChildren,
    ) -> Result<Vec<data::Block>> {
        let response: Result<data::Object, _> = self
            .request(Method::GET, &f!("blocks/{block_id}/children"))
            .query(&query)
            .send()
            .await?
            .json()
            .await;

        let Ok(response) = response else {
            // TODO: this is jank. Fix.
            return Ok(vec![]);
        };

        let data::Object::List(data::List { results }) = response else {
            bail!("Result {response:?} is not a list")
        };

        let res: Vec<_> = results
            .into_iter()
            .map(|result| {
                let data::Object::Block(block) = result else {
                    bail!("Result {result:?} is not a block")
                };
                Ok(*block)
            })
            .try_collect()?;
        Ok(res)
    }

    /// # Errors
    /// - If the request fails.
    /// - If the response is not a page.
    #[instrument(skip(self), fields(page_id = %page_id))]
    pub async fn get_page(&self, page_id: &str) -> Result<data::Page> {
        let response: data::Object = self
            .request(Method::GET, &f!("pages/{page_id}"))
            .send()
            .await?
            .json()
            .await?;

        let data::Object::Page(page) = response else {
            bail!("Result {response:?} is not a page")
        };

        Ok(page)
    }

    /// # Errors
    /// - If the request fails.
    #[instrument(skip(self), fields(query = %query))]
    pub async fn search(&self, query: &str) -> Result<SearchResponse> {
        use data::SearchRequest;

        let query = match query {
            "" => None,
            _ => Some(query),
        };

        let req = SearchRequest { query, ..default() };

        let response = self
            .request(Method::POST, "search")
            .json(&req)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;

    use crate::{data::BlockData, default, Client};

    static API: Lazy<Client> =
        Lazy::new(|| Client::new(std::env::var("NOTION_ACCESS_TOKEN").unwrap()));

    static MEETING_PAGE_ID: Lazy<Box<str>> = Lazy::new(|| {
        let s = std::env::var("NOTION_MEETING_PAGE_ID").unwrap();
        Box::from(s)
    });

    #[tokio::test]
    async fn test_list_users() {
        let res = API.list_users().await.unwrap();
        println!("{res:#?}");
    }

    #[tokio::test]
    async fn test_get_page() {
        let res = API.get_page(&MEETING_PAGE_ID).await.unwrap();
        println!("{res:#?}");
    }

    #[tokio::test]
    async fn test_block_children() {
        let res = API
            .block_children(&MEETING_PAGE_ID, default())
            .await
            .unwrap();

        let headers = res
            .into_iter()
            .map(|block| block.data)
            .filter_map(BlockData::into_heading);

        for header in headers {
            println!("{header:?}");
        }
    }

    #[tokio::test]
    async fn test_retrieve_block() {
        let res = API.block(&MEETING_PAGE_ID).await.unwrap();
        println!("{res:#?}");
    }

    #[tokio::test]
    async fn test_search() {
        API.search("").await.unwrap();
    }
}
