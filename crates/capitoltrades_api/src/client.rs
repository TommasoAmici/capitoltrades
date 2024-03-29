use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    query::{IssuerQuery, PoliticianQuery, Query, TradeQuery},
    types::{IssuerDetail, PoliticianDetail, Response, Trade},
    user_agent::get_user_agent,
    Error,
};

pub struct Client {
    base_api_url: &'static str,
}

impl Client {
    pub fn new() -> Self {
        Self {
            base_api_url: "https://bff.capitoltrades.com",
        }
    }

    fn get_url(&self, path: &str, query: &impl Query) -> Url {
        let mut url = Url::parse(format!("{}{}", &self.base_api_url, path).as_str()).unwrap();
        query.add_to_url(&mut url)
    }

    async fn get<T, Q>(&self, path: &str, query: &Q) -> Result<Response<T>, Error>
    where
        T: DeserializeOwned,
        Q: Query,
    {
        let url = self.get_url(path, query);
        let client = reqwest::Client::builder()
            .user_agent(get_user_agent())
            .build()
            .map_err(|e| {
                tracing::error!("Failed to build HTTP client: {}", e);
                Error::RequestFailed
            })?;
        let resp = client
            .get(url)
            .header("content-type", "application/json")
            .header("origin", "https://www.capitoltrades.com")
            .header("referer", "https://www.capitoltrades.com")
            .header("accept", "*/*")
            .header("accept-language", "en-US,en;q=0.9")
            .header("sec-fetch-dest", "empty")
            .header("sec-fetch-mode", "cors")
            .header("sec-fetch-site", "same-site")
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to get resource: {}", e);
                Error::RequestFailed
            })?
            .json::<Response<T>>()
            .await
            .map_err(|e| {
                tracing::error!("Failed to parse resource: {}", e);
                Error::RequestFailed
            })?;
        Ok(resp)
    }

    pub async fn get_trades(&self, query: &TradeQuery) -> Result<Response<Trade>, Error> {
        self.get::<Trade, TradeQuery>("/trades", query).await
    }

    pub async fn get_politicians(
        &self,
        query: &PoliticianQuery,
    ) -> Result<Response<PoliticianDetail>, Error> {
        self.get::<PoliticianDetail, PoliticianQuery>("/politicians", query)
            .await
    }

    pub async fn get_issuers(&self, query: &IssuerQuery) -> Result<Response<IssuerDetail>, Error> {
        self.get::<IssuerDetail, IssuerQuery>("/issuers", query)
            .await
    }
}
