use crate::credentials::Credentials;
use reqwest::Client;
use reqwest::Url;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;

pub struct DeltaForceSdk<'a> {
    pub endpoint: Url,
    pub credentials: Option<Credentials<'a>>,
    pub client: Client,
}

impl<'a> DeltaForceSdk<'a> {
    pub fn build() -> DeltaForceSdkBuilder<'a> {
        DeltaForceSdkBuilder::new()
    }
}

pub struct DeltaForceSdkBuilder<'a> {
    endpoint: Url,
    credentials: Option<Credentials<'a>>,
    client: Client,
}

impl<'a> DeltaForceSdkBuilder<'a> {
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

    pub fn with_credentials(mut self, x: Credentials<'a>) -> Self {
        self.credentials = Some(x);
        self
    }

    pub fn build(self) -> DeltaForceSdk<'a> {
        DeltaForceSdk {
            endpoint: self.endpoint,
            credentials: self.credentials,
            client: self.client,
        }
    }
}
