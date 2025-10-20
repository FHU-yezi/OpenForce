use std::io::stdin;

use crate::error::Error;
use df_sdk::credentials::Credentials;
use std::fs;
use time::{Date, PrimitiveDateTime, Time, macros::format_description};

use crate::Cli;

pub fn get_credentials(cli: &Cli) -> Result<Credentials, Error> {
    let mut cookies_providers_count = 0;
    if cli.cookies.is_some() {
        cookies_providers_count += 1;
    };
    if cli.cookies_file.is_some() {
        cookies_providers_count += 1;
    };
    if cli.cookies_stdin {
        cookies_providers_count += 1;
    }
    if cookies_providers_count == 0 {
        return Err(Error::InvaildArgument(
            "必须指定一个 Cookies 来源".to_string(),
        ));
    }
    if cookies_providers_count > 1 {
        return Err(Error::InvaildArgument(
            "不能同时指定多个 Cookies 来源".to_string(),
        ));
    }

    let mut cookies = String::new();

    if let Some(x) = &cli.cookies {
        cookies = x.clone();
    }

    if let Some(x) = &cli.cookies_file {
        cookies = fs::read_to_string(x)
            .map_err(|e| Error::InvaildArgument(format!("读取 Cookies 文件失败：{e}")))?;
    }

    if cli.cookies_stdin {
        let mut x = String::new();
        stdin()
            .read_line(&mut x)
            .map_err(|e| Error::InvaildArgument(format!("从标准输入读取 Cookies 失败：{e}")))?;
        cookies = x;
    }

    Credentials::from_cookies(&cookies).map_err(|_| Error::InvalidCredentials)
}

pub fn parse_datetime(x: &str) -> Option<PrimitiveDateTime> {
    let formats = [
        format_description!("[year]-[month]-[day] [hour]:[minute]"),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
        format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]"),
    ];

    // 尝试解析完整的日期时间格式
    for format in formats {
        if let Ok(result) = PrimitiveDateTime::parse(x, format) {
            return Some(result);
        }
    }

    // 尝试解析日期格式，添加默认时间
    if let Ok(date) = Date::parse(x, format_description!("[year]-[month]-[day]")) {
        let time = Time::from_hms(0, 0, 0).unwrap();
        return Some(PrimitiveDateTime::new(date, time));
    }

    None
}
