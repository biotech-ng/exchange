use serde::ser::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::format_description::well_known::Iso8601;
use time::{OffsetDateTime, PrimitiveDateTime};

pub const DATE_TIME_FORMAT: Iso8601 = Iso8601::DEFAULT;

fn serialize_date<S>(v: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    v.format(&DATE_TIME_FORMAT)
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
    value: OffsetDateTime,
}

impl From<PrimitiveDateTime> for JsonDateTime {
    fn from(value: PrimitiveDateTime) -> Self {
        JsonDateTime {
            value: value.assume_utc(),
        }
    }
}

impl<'de> Deserialize<'de> for JsonDateTime {
    fn deserialize<D>(deserializer: D) -> Result<JsonDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_date(deserializer).map(Into::into)
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

#[cfg(test)]
mod tests {
    use super::*;
    use time::OffsetDateTime;

    #[test]
    fn test_serialize_deserialize_date() {
        let now = OffsetDateTime::now_utc();
        let serialized = now.format(&DATE_TIME_FORMAT).expect("valid format");

        let deserialized_now =
            OffsetDateTime::parse(serialized.as_str(), &DATE_TIME_FORMAT).expect("valid date");

        assert_eq!(deserialized_now, now);
    }
}
