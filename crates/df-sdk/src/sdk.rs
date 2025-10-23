use crate::credentials::Credentials;
use reqwest::Client;
use reqwest::Url;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;

pub struct DeltaForceSdk {
    pub endpoint: Url,
    pub credentials: Option<Credentials>,
    pub client: Client,
}

impl DeltaForceSdk {
    pub fn build() -> DeltaForceSdkBuilder {
        DeltaForceSdkBuilder::new()
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
