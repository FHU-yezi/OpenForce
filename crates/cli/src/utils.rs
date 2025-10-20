use std::io::{Write, stdin, stdout};

use time::{Date, PrimitiveDateTime, Time, macros::format_description};

// TODO: 支持从命令行参数、文件中传递 Cookies
pub fn get_cookies() -> String {
    print!("请输入 Cookies：");
    stdout().flush().unwrap();

    let mut cookies = String::new();
    stdin().read_line(&mut cookies).unwrap();

    cookies
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
