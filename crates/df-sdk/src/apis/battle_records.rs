use serde_json::Value;

use crate::error::Error;
use crate::sdk::DeltaForceSdk;

use crate::apis::send_api_request;

pub async fn get_battle_records_list_api(
    sdk: &DeltaForceSdk,
    page: u8,
) -> Result<Vec<Value>, Error> {
    Ok(send_api_request(
        sdk,
        &[
            ("iChartId", "450526"),
            ("iSubChartId", "450526"),
            ("sIdeToken", "PHq59Y"),
            ("type", "4"),
            ("page", &page.to_string()),
        ],
        true,
    )
    .await?
    .as_array()
    .ok_or(Error::ParseError)?
    .clone())
}

pub async fn get_battle_record_details_api(
    sdk: &DeltaForceSdk,
    room_id: &str,
) -> Result<Vec<Value>, Error> {
    Ok(send_api_request(
        sdk,
        &[
            ("iChartId", "450471"),
            ("iSubChartId", "450471"),
            ("sIdeToken", "ylP3eG"),
            ("roomId", room_id),
            ("type", "2"),
        ],
        true,
    )
    .await?
    .as_array()
    .ok_or(Error::ParseError)?
    .clone())
}
