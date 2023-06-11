use serde::ser::Error;
use serde::{Deserialize, Deserializer, Serializer};
use time::format_description::FormatItem;
use time::macros::format_description;
use time::PrimitiveDateTime;

const DATE_TIME_FORMAT: &[FormatItem<'static>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

pub fn date_as_string<S>(v: &PrimitiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let str = v
        .format(DATE_TIME_FORMAT)
        .map_err(|err| Error::custom(std::format!("failed to serialize date: {err}")))?;

    serializer.serialize_str(str.as_str())
}

pub fn string_as_date<'de, D>(deserializer: D) -> Result<PrimitiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    String::deserialize(deserializer).and_then(|string| {
        PrimitiveDateTime::parse(string.as_str(), &DATE_TIME_FORMAT)
            .map_err(|err| Error::custom(std::format!("failed to deserialize date: {err}")))
    })
}
