use std::sync::Arc;

const API_BASE: &str = "https://api.dropboxapi.com/2/";

/// Error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

/// The Client
#[derive(Debug, Clone)]
pub struct Client {
    /// The inner client
    pub client: reqwest::Client,

    /// The token
    pub token: Arc<str>,
}

impl Client {
    /// Make a new client from a token.
    pub fn new(token: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {token}").parse().unwrap(),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );

        Self {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
            token: token.into(),
        }
    }

    /// List shared folders
    pub async fn sharing_list_folders(
        &self,
        args: &ListFoldersArgs,
    ) -> Result<ListFoldersResult, Error> {
        let json = self
            .client
            .post(format!("{API_BASE}sharing/list_folders"))
            .json(&args)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(json)
    }

    pub async fn list_folder(&self, args: &ListFolderArg) -> Result<ListFoldersResult, Error> {
        let json = self
            .client
            .post(format!("{API_BASE}files/list_folder"))
            .json(&args)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(json)
    }
}

/// Args for listing shared folders
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListFoldersArgs {}

/// The result for listing shared folders
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListFoldersResult {
    /// The cursor, if there are more results
    pub cursor: Option<Box<str>>,

    /// List of shared folders
    pub entries: Vec<serde_json::Value>,
}

/// Args for listing a folder
#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListFolderArg {
    /// The folder path
    pub path: Box<str>,
}

/// The result for listing a folder
pub struct ListFolderResult {
    /// The cursor, if there are more results
    pub cursor: Option<Box<str>>,

    /// List of files and folders
    pub entries: Vec<serde_json::Value>,

    /// Whether more entries are available
    pub has_more: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    const TOKEN: &str = include_str!("../token.txt");

    #[tokio::test]
    async fn sharing_list_folders() {
        let client = Client::new(TOKEN);
        let results = client
            .sharing_list_folders(&Default::default())
            .await
            .expect("failed to list folders");
        dbg!(&results);
    }

    #[tokio::test]
    async fn list_folder() {
        let client = Client::new(TOKEN);
        let results = client
            .list_folder(&ListFolderArg { path: "".into() })
            .await
            .expect("failed to list folders");
        dbg!(&results);
    }
}
