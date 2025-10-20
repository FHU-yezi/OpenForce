use crate::error::Error;

pub struct Credentials {
    account_type: String,
    open_id: String,
    access_token: String,
}

impl Credentials {
    pub fn from_cookies(cookies_string: &str) -> Result<Self, Error> {
        let mut account_type: Option<&str> = None;
        let mut open_id: Option<&str> = None;
        let mut access_token: Option<&str> = None;

        for pair in cookies_string.split(";") {
            if let Some(equal_sign_index) = pair.find("=") {
                let (key, value) = pair.split_at(equal_sign_index);
                let key = key.trim();
                let value = value[1..].trim(); // 跳过等于号本身
                match key {
                    "acctype" => account_type = Some(value),
                    "openid" => open_id = Some(value),
                    "access_token" => access_token = Some(value),
                    _ => (),
                }
            }
        }

        if let (Some(account_type), Some(open_id), Some(access_token)) =
            (account_type, open_id, access_token)
        {
            Ok(Self {
                account_type: account_type.to_string(),
                open_id: open_id.to_string(),
                access_token: access_token.to_string(),
            })
        } else {
            Err(Error::InvalidCredentials)
        }
    }

    pub fn to_cookies(&self) -> String {
        format!(
            "acctype={};openid={};access_token={}",
            self.account_type, self.open_id, self.access_token
        )
    }
}
