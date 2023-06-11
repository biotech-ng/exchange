use serde::ser::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::format_description::FormatItem;
use time::macros::format_description;
use time::PrimitiveDateTime;

pub const DATE_TIME_FORMAT: &[FormatItem<'static>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

fn serialize_date<S>(v: &PrimitiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    v.format(DATE_TIME_FORMAT)
        .map_err(|err| Error::custom(std::format!("failed to serialize date: {err}")))
        .and_then(|str| serializer.serialize_str(str.as_str()))
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<PrimitiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    String::deserialize(deserializer).and_then(|string| {
        PrimitiveDateTime::parse(string.as_str(), &DATE_TIME_FORMAT)
            .map_err(|err| Error::custom(std::format!("failed to deserialize date: {err}")))
    })
}

#[derive(Debug)]
pub struct JsonDateTime {
    value: PrimitiveDateTime,
}

impl From<PrimitiveDateTime> for JsonDateTime {
    fn from(value: PrimitiveDateTime) -> Self {
        JsonDateTime { value }
    }
}

impl<'de> Deserialize<'de> for JsonDateTime {
    fn deserialize<D>(deserializer: D) -> Result<JsonDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_date(deserializer).map(|x| JsonDateTime { value: x })
    }
}

impl Serialize for JsonDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize_date(&self.value, serializer)
    }
}
