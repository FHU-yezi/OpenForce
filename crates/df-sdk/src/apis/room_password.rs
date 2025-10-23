use serde_json::Value;

use crate::apis::send_api_request;
use crate::error::Error;
use crate::sdk::DeltaForceSdk;

pub async fn get_room_password_list_api(sdk: &DeltaForceSdk) -> Result<Vec<Value>, Error> {
    Ok(send_api_request(
        sdk,
        &[
            ("iChartId", "316969"),
            ("iSubChartId", "316969"),
            ("sIdeToken", "NoOapI"),
            ("method", "dfm/center.day.secret"),
            ("source", "2"),
        ],
        true,
    )
    .await?["data"]
        .as_object()
        .ok_or(Error::ParseError)?["list"]
        .as_array()
        .ok_or(Error::ParseError)?
        .clone())
}
