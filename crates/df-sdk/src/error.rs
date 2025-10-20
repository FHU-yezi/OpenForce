use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("鉴权凭证无效")]
    InvalidCredentials,

    #[error("需要鉴权凭证")]
    MissingCredentials,

    #[error("网络请求异常：{0}")]
    RequestError(reqwest::Error),

    #[error("网络请求失败：HTTP Status {0}")]
    HttpStatusError(reqwest::StatusCode),

    #[error("反序列化异常：{0}")]
    DeserializeError(reqwest::Error),

    #[error("API 状态码异常：{0}")]
    ApiStatusError(String),

    #[error("数据解析异常")]
    ParseError,

    #[error("未知数据：{0}")]
    UnknownData(String),
}
