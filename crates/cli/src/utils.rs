use std::io::stdin;

use crate::error::Error;
use df_sdk::credentials::Credentials;
use std::env;
use std::fs;
use time::OffsetDateTime;
use time::{Date, PrimitiveDateTime, Time, macros::format_description};

use crate::CookiesArgs;

const COOKIES_ENV_NAME: &str = "OPENFORCE_COOKIES";

fn get_credentials_from_env() -> Result<Credentials, Error> {
    match env::var(COOKIES_ENV_NAME) {
        Ok(cookies) => Credentials::from_cookies(&cookies).map_err(|_| {
            Error::InvalidCredentials(format!("从环境变量 {COOKIES_ENV_NAME} 读取的 Cookies 无效"))
        }),
        Err(env::VarError::NotPresent) => Err(Error::MissingCredentials(format!(
            "未设置环境变量 {COOKIES_ENV_NAME}"
        ))),
        Err(_) => Err(Error::InvalidCredentials(format!(
            "读取环境变量 {COOKIES_ENV_NAME} 时发生未知异常"
        ))),
    }
}

fn get_credentials_from_file(path: &str) -> Result<Credentials, Error> {
    match fs::read_to_string(path) {
        Ok(cookies) => Credentials::from_cookies(&cookies)
            .map_err(|_| Error::InvalidCredentials(format!("从文件 {path} 读取的 Cookies 无效"))),
        Err(_) => Err(Error::InvalidCredentials(format!(
            "读取文件 {path} 时发生未知异常"
        ))),
    }
}

fn get_credentials_from_stdin() -> Result<Credentials, Error> {
    let mut cookies = String::new();

    match stdin().read_line(&mut cookies) {
        Ok(_) => Credentials::from_cookies(&cookies)
            .map_err(|_| Error::InvalidCredentials("从标准输入读取的 Cookies 无效".to_string())),
        Err(_) => Err(Error::InvalidCredentials(
            "读取标准输入时发生未知异常".to_string(),
        )),
    }
}

pub fn get_credentials(cookies_args: &CookiesArgs) -> Result<Credentials, Error> {
    // 如果用户提供了具体的 Cookies 来源，仅从对应来源读取
    // 否则，仅从环境变量读取
    if cookies_args.cookies_env {
        get_credentials_from_env()
    } else if let Some(path) = &cookies_args.cookies_file {
        get_credentials_from_file(path)
    } else if cookies_args.cookies_stdin {
        get_credentials_from_stdin()
    } else {
        get_credentials_from_env()
    }
}

pub fn parse_datetime(x: &str) -> Result<PrimitiveDateTime, String> {
    let date_format = format_description!("[year]-[month]-[day]");
    let formats = [
        format_description!("[year]-[month]-[day] [hour]:[minute]"),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
        format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]"),
    ];

    // 尝试解析完整的日期时间格式
    for format in formats {
        if let Ok(result) = PrimitiveDateTime::parse(x, format) {
            return Ok(result);
        }
    }

    // 尝试解析日期格式，添加默认时间
    if let Ok(date) = Date::parse(x, date_format) {
        let time = Time::from_hms(0, 0, 0).unwrap();
        return Ok(PrimitiveDateTime::new(date, time));
    }

    let time_now = OffsetDateTime::now_local().unwrap();
    let all_formats = [date_format]
        .into_iter()
        .chain(formats.into_iter())
        .collect::<Vec<_>>();
    let supported_format_examples = all_formats
        .iter()
        .map(|format| time_now.format(format).unwrap())
        .collect::<Vec<_>>()
        .join("\n    - ");

    Err(format!(
        r#"
支持的时间格式：
    - {}"#,
        supported_format_examples
    ))
}
