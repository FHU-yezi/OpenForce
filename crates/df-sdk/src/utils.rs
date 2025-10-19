use reqwest::Response;
use serde_json::Value;

use crate::error::Error;

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
