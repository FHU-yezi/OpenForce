use reqwest::Response;
use serde_json::Value;

pub async fn extract_data(response: Response) -> Result<Value, String> {
    if !response.status().is_success() {
        return Err(format!("请求失败：HTTP Status {}", response.status()));
    }

    let body: Value = response
        .json()
        .await
        .map_err(|e| format!("解析数据失败：{e}"))?;

    if body["ret"].as_u64().ok_or("解析数据失败".to_string())? != 0
        || body["iRet"].as_u64().ok_or("解析数据失败".to_string())? != 0
    {
        return Err(format!(
            "请求失败：{}",
            body["sMsg"].as_str().ok_or("解析数据失败".to_string())?
        ));
    }

    body["jData"]
        .as_object()
        .ok_or("解析数据失败".to_string())
        .and_then(|x| Ok(x["data"].clone()))
}
