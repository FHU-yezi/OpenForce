use crate::credentials::Credentials;
use crate::error::Error;
use crate::utils::extract_data;
use reqwest::Client;
use reqwest::Url;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use serde_json::Value;

pub struct DeltaForceSdk {
    pub endpoint: Url,
    pub credentials: Option<Credentials>,
    pub client: Client,
}

impl DeltaForceSdk {
    pub fn build() -> DeltaForceSdkBuilder {
        DeltaForceSdkBuilder::new()
    }

    pub async fn send_api_request(&self, query_params: &[(&str, &str)]) -> Result<Value, Error> {
        let mut url = self.endpoint.clone();

        {
            let mut query_pairs = url.query_pairs_mut();
            for (key, value) in query_params {
                query_pairs.append_pair(key, value);
            }
        }

        let request = self.client.post(url).header(
            "Cookie",
            self.credentials
                .as_ref()
                .ok_or(Error::MissingCredentials)?
                .to_cookies(),
        );

        let response = request.send().await.map_err(|e| Error::RequestError(e))?;

        Ok(extract_data(response).await?)
    }
}

pub struct DeltaForceSdkBuilder {
    endpoint: Url,
    credentials: Option<Credentials>,
    client: Client,
}

impl DeltaForceSdkBuilder {
    pub fn new() -> Self {
        Self {
            endpoint: Url::parse("https://comm.ams.game.qq.com/ide/").unwrap(),
            credentials: None,
            client: {
                let mut headers = HeaderMap::new();
                headers.insert(
                    "Content-Type",
                    HeaderValue::from_static("application/x-www-form-urlencoded"),
                );

                Client::builder().default_headers(headers).build().unwrap()
            },
        }
    }

    pub fn endpoint(mut self, x: &str) -> Self {
        self.endpoint = Url::parse(x).unwrap();
        self
    }

    pub fn with_credentials(mut self, x: Credentials) -> Self {
        self.credentials = Some(x);
        self
    }

    pub fn build(self) -> DeltaForceSdk {
        DeltaForceSdk {
            endpoint: self.endpoint,
            credentials: self.credentials,
            client: self.client,
        }
    }
}
