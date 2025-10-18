pub struct Credentials {
    account_type: String,
    open_id: String,
    access_token: String,
}

impl Credentials {
    pub fn from_cookies(cookies_string: &str) -> Result<Self, String> {
        let mut account_type = String::new();
        let mut open_id = String::new();
        let mut access_token = String::new();

        for pair in cookies_string.split(";") {
            if let Some(equal_sign_index) = pair.find("=") {
                let (key, value) = pair.split_at(equal_sign_index);
                let key = key.trim().to_string();
                let value = value[1..].trim().to_string(); // 跳过等于号本身
                match key.as_str() {
                    "acctype" => account_type = value,
                    "openid" => open_id = value,
                    "access_token" => access_token = value,
                    _ => (),
                }
            }
        }
        if account_type.len() == 0 || open_id.len() == 0 || access_token.len() == 0 {
            return Err("Cookies 缺少鉴权信息".into());
        }

        Ok(Self {
            account_type,
            open_id,
            access_token,
        })
    }
}
