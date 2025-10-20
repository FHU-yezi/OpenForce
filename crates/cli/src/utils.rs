use std::io::stdin;

use df_sdk::credentials::Credentials;
use df_sdk::error::Error as SdkError;
use std::fs;
use time::{Date, PrimitiveDateTime, Time, macros::format_description};

use crate::Cli;

pub fn get_credentials(cli: &Cli) -> Result<Credentials, SdkError> {
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
        panic!("没有 Cookies 来源");
    }
    if cookies_providers_count > 1 {
        panic!("不能同时指定多个 Cookies 来源");
    }

    if let Some(cookies) = &cli.cookies {
        return Credentials::from_cookies(cookies);
    }

    if let Some(cookies_file) = &cli.cookies_file {
        let cookies = fs::read_to_string(cookies_file).expect("读取 Cookies 文件失败");
        return Credentials::from_cookies(&cookies);
    }

    if cli.cookies_stdin {
        let mut cookies = String::new();
        stdin()
            .read_line(&mut cookies)
            .expect("从标准输入读取 Cookies 失败");
        return Credentials::from_cookies(&cookies);
    }

    unreachable!();
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
