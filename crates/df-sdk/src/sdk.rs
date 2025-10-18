use crate::credentials::Credentials;
use reqwest::Client;
use reqwest::Url;

pub struct DeltaForceSdk<'a> {
    pub base_url: Url,
    pub credentials: Option<Credentials<'a>>,
    pub client: Client,
}

impl<'a> DeltaForceSdk<'a> {
    pub fn build() -> DeltaForceSdkBuilder<'a> {
        DeltaForceSdkBuilder::new()
    }
}

pub struct DeltaForceSdkBuilder<'a> {
    base_url: Url,
    credentials: Option<Credentials<'a>>,
    client: Client,
}

impl<'a> DeltaForceSdkBuilder<'a> {
    pub fn new() -> Self {
        Self {
            base_url: Url::parse("https://comm.ams.game.qq.com").unwrap(),
            credentials: None,
            client: Client::new(),
        }
    }

    pub fn base_url(mut self, x: &str) -> Self {
        self.base_url = Url::parse(x).unwrap();
        self
    }

    pub fn with_credentials(mut self, x: Credentials<'a>) -> Self {
        self.credentials = Some(x);
        self
    }

    pub fn build(self) -> DeltaForceSdk<'a> {
        DeltaForceSdk {
            base_url: self.base_url,
            credentials: self.credentials,
            client: self.client,
        }
    }
}
