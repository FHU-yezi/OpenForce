use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("缺少鉴权凭证：{0}")]
    MissingCredentials(String),

    #[error("鉴权凭证无效：{0}")]
    InvalidCredentials(String),

    #[error("序列化异常：{0}")]
    SerializeError(serde_json::Error),
}
