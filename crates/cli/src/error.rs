use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("无效参数：")]
    InvaildArgument(String),

    #[error("鉴权凭证无效")]
    InvalidCredentials,

    #[error("序列化异常：{0}")]
    SerializeError(serde_json::Error),
}
