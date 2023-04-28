use std::{collections::HashMap, sync::Arc};

use data::parent_object;
use parking_lot::RwLock;

use crate::{data, data::Page, Client, default};

pub struct CachedClient {
    /// The pages that are currently cached.
    ///
    /// Map: ID -> Page
    pages: RwLock<HashMap<String, Arc<Page>>>,
    client: Client,
}

impl CachedClient {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self {
            pages: default(),
            client,
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// # Errors
    /// If the request fails.
    pub async fn search(&self, term: &str) -> anyhow::Result<Vec<Arc<Page>>> {
        let response = self.client.search(term).await?;
        let data::SearchResponse { results, .. } = response;

        let mut res = Vec::with_capacity(results.len());
        for result in results {
            let result = Arc::new(result);
            self.pages.write().insert(result.id.clone(), result.clone());
            res.push(result);
        }
        Ok(res)
    }

    pub async fn get_page(&self, id: &str) -> Option<Arc<Page>> {
        // TODO: remove when possible
        // https://www.reddit.com/r/rust/comments/wbacbf/comment/ii5pcby/
        if self.pages.read().contains_key(id) {
            return self.pages.read().get(id).cloned();
        }

        let Ok(page) = self.client.get_page(id).await else { return None };

        self.pages.write().insert(id.to_string(), page.into());

        self.pages.read().get(id).cloned()
    }

    /// Get all pages that are currently cached.
    pub fn pages(&self) -> Vec<Arc<Page>> {
        self.pages.read().values().cloned().collect()
    }

    /// Get path of the page as
    ///
    /// `/<parent>/<parent>/<parent>/<page>`
    pub async fn get_path(&self, id: &str) -> Option<Vec<String>> {
        let page = self.get_page(id).await?;

        // ids in reverse
        let mut ids = vec![page.title()?.plain_text.clone()];

        let mut page_on = page;
        while let Some(parent) = page_on.parent.clone() {
            let parent_object::Data::PageId(parent_id) = parent.data else {
                return Some(reverse(ids));
            };
            let Some(parent) = self.get_page(&parent_id).await else {
                return Some(reverse(ids));
            };
            let parent_title = parent.title()?.plain_text.clone();
            ids.push(parent_title);
            page_on = parent;
        }

        ids.reverse();

        Some(ids)
    }

    /// Get path of the page similar to `get_path` but a single output string that looks
    pub async fn get_path_current(&self, id: &str) -> Option<String> {
        let path = self.get_path(id).await?;
        path.into_iter().last()
        // let res = path.into_iter().join(" -> ");
    }
}

/// Reverse the list in place and return it.
fn reverse(list: Vec<String>) -> Vec<String> {
    let mut list = list;
    list.reverse();
    list
}

#[cfg(test)]
mod tests {
    use crate::{utils::CachedClient, Client};

    fn api() -> CachedClient {
        let client = Client::new(std::env::var("NOTION_ACCESS_TOKEN").unwrap());
        CachedClient::new(client)
    }

    const MEETING_PAGE_ID: &str = "c2d57097-582a-4ddc-b2e7-9c7f64c21923";

    #[tokio::test]
    async fn test_get_path() {
        let api = api();
        let path = api.get_path_current(MEETING_PAGE_ID).await.unwrap();
        println!("{path}");
    }
}
