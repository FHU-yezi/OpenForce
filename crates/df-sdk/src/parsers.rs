use serde_json::Value;
use time::{PrimitiveDateTime, macros::format_description};

pub fn parse_str(x: &Value) -> Result<String, String> {
    x.as_str()
        .ok_or("转换 str 类型失败".to_string())
        .map(|x| x.to_string())
}

pub fn parse_int<T>(x: &Value) -> Result<T, String>
where
    T: TryFrom<i64>,
    <T as TryFrom<i64>>::Error: std::fmt::Display,
{
    x.as_i64()
        .ok_or("转换 int 类型失败".to_string())
        .and_then(|x| T::try_from(x).map_err(|e| format!("转换 int 类型失败：{e}")))
}

pub fn parse_uint<T>(x: &Value) -> Result<T, String>
where
    T: TryFrom<u64>,
    <T as TryFrom<u64>>::Error: std::fmt::Display,
{
    x.as_u64()
        .ok_or("转换 uint 类型失败".to_string())
        .and_then(|x| T::try_from(x).map_err(|e| format!("转换 uint 类型失败：{e}")))
}

pub fn parse_str_then_number<T>(x: &Value) -> Result<T, String>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    x.as_str()
        .ok_or("转换 str 类型失败".to_string())
        .and_then(|x| {
            x.parse::<T>()
                .map_err(|e| format!("转换 str 类型失败：{e}"))
        })
}

pub fn parse_time(x: &Value) -> Result<PrimitiveDateTime, String> {
    let time_format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

    PrimitiveDateTime::parse(x.as_str().ok_or("转换 str 类型失败")?, time_format)
        .map_err(|e| format!("解析日期失败：{e}"))
}
