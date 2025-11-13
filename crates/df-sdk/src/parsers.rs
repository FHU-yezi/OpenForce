use crate::constants::{escape_result::EscapeResult, level::Level, map::Map, operator::Operator};
use serde_json::Value;
use time::{PrimitiveDateTime, macros::format_description};

use crate::error::Error;

pub fn parse_str(x: &Value) -> Result<String, Error> {
    x.as_str().ok_or(Error::ParseError).map(|x| x.to_string())
}

pub fn parse_int<T>(x: &Value) -> Result<T, Error>
where
    T: TryFrom<i64>,
    <T as TryFrom<i64>>::Error: std::fmt::Display,
{
    x.as_i64()
        .ok_or(Error::ParseError)
        .and_then(|x| T::try_from(x).map_err(|_| Error::ParseError))
}

pub fn parse_uint<T>(x: &Value) -> Result<T, Error>
where
    T: TryFrom<u64>,
    <T as TryFrom<u64>>::Error: std::fmt::Display,
{
    x.as_u64()
        .ok_or(Error::ParseError)
        .and_then(|x| T::try_from(x).map_err(|_| Error::ParseError))
}

pub fn parse_str_then_number<T>(x: &Value) -> Result<T, Error>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    x.as_str()
        .ok_or(Error::ParseError)
        .and_then(|x| x.parse::<T>().map_err(|_| Error::ParseError))
}

pub fn parse_time(x: &Value) -> Result<PrimitiveDateTime, Error> {
    let time_format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

    PrimitiveDateTime::parse(x.as_str().ok_or(Error::ParseError)?, time_format)
        .map_err(|_| Error::ParseError)
}

pub fn parse_map_id_to_map(x: &Value) -> Result<Map, Error> {
    let map_id = parse_str_then_number(x)?;

    Map::from_map_id(map_id).ok_or(Error::UnknownData(format!("未知的地图 ID（{map_id}）")))
}

pub fn parse_map_id_to_level(x: &Value) -> Result<Level, Error> {
    let map_id = parse_str_then_number(x)?;

    Level::from_map_id(map_id).ok_or(Error::UnknownData(format!("未知的地图 ID（{map_id}）")))
}

pub fn parse_operator_id(x: &Value) -> Result<Operator, Error> {
    let operator_id = parse_uint(x)?;

    Operator::from_operator_id(operator_id).ok_or(Error::UnknownData(format!(
        "未知的干员 ID（{operator_id}）"
    )))
}

pub fn parse_escape_result(x: &Value) -> Result<EscapeResult, Error> {
    let escape_result_id = parse_uint(x)?;

    EscapeResult::from_escape_result_id(escape_result_id).ok_or(Error::UnknownData(format!(
        "未知的撤离结果 ID（{escape_result_id}）"
    )))
}
