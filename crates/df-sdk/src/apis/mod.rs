pub mod battle_records;
pub mod room_password;

use reqwest::Response;
use serde_json::Value;

use crate::error::Error;
use crate::sdk::DeltaForceSdk;

pub async fn extract_data(response: Response) -> Result<Value, Error> {
    if !response.status().is_success() {
        return Err(Error::HttpStatusError(response.status()));
    }

    let body: Value = response
        .json()
        .await
        .map_err(|e| Error::DeserializeError(e))?;

    if body["ret"].as_u64().ok_or(Error::ParseError)? != 0
        || body["iRet"].as_u64().ok_or(Error::ParseError)? != 0
    {
        return Err(Error::ApiStatusError(
            body["sMsg"].as_str().ok_or(Error::ParseError)?.to_string(),
        ));
    }

    body["jData"]
        .as_object()
        .ok_or(Error::ParseError)
        .and_then(|x| Ok(x["data"].clone()))
}

pub async fn send_api_request(
    sdk: &DeltaForceSdk,
    query_params: &[(&str, &str)],
    with_credentials: bool,
) -> Result<Value, Error> {
    let mut url = sdk.endpoint.clone();

    {
        let mut query_pairs = url.query_pairs_mut();
        for (key, value) in query_params {
            query_pairs.append_pair(key, value);
        }
    }

    let mut request = sdk.client.post(url);
    if with_credentials {
        request = request.header(
            "Cookie",
            sdk.credentials
                .as_ref()
                .ok_or(Error::MissingCredentials)?
                .to_cookies(),
        );
    }

    let response = request.send().await.map_err(|e| Error::RequestError(e))?;

    Ok(extract_data(response).await?)
}
