use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use uuid::Uuid;

fn serialize_system_time<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let duration = time
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0));
    serializer.serialize_i64(duration.as_secs() as i64)
}

#[derive(Clone, Deserialize, Serialize)]
pub struct XPayload {
    pub ByteField: u8,
    pub SByteField: i8,
    pub ShortField: i16,
    pub UShortField: u16,
    pub IntField: i32,
    pub UIntField: u32,
    pub LongField: i64,
    pub ULongField: u64,

    pub FloatField: f32,
    pub DoubleField: f64,
    pub DecimalField: f64, // Rust não possui decimal nativo, pode-se usar f64 ou biblioteca como `rust_decimal`

    pub BoolField: bool,
    pub CharField: char,
    pub StringField: String,

    pub GuidField: Uuid,

    // #[serde(serialize_with = "serialize_iso8601", deserialize_with = "deserialize_iso8601")]
    // pub DateTimeField: DateTime<Utc>,
    // #[serde(serialize_with = "serialize_timespan", deserialize_with = "deserialize_timespan")]
    // pub TimeSpanField: Duration,

    pub Persons: Vec<XPersonData>,
}

fn serialize_timespan<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let secs = duration.as_secs();
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;
    let timespan_str = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
    serializer.serialize_str(&timespan_str)
}

fn deserialize_timespan<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 3 {
        return Err(serde::de::Error::custom("Expected format hh:mm:ss"));
    }

    let hours = parts[0].parse::<u64>().map_err(serde::de::Error::custom)?;
    let minutes = parts[1].parse::<u64>().map_err(serde::de::Error::custom)?;
    let seconds = parts[2].parse::<u64>().map_err(serde::de::Error::custom)?;

    Ok(Duration::from_secs(hours * 3600 + minutes * 60 + seconds))
}
fn serialize_iso8601<S>(time: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where S: serde::Serializer {
    serializer.serialize_str(&time.to_rfc3339())
}

fn deserialize_iso8601<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where D: serde::Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    s.parse::<DateTime<Utc>>().map_err(serde::de::Error::custom)
}
#[derive(Clone, Deserialize, Serialize)]
pub struct XPersonData {
    pub Id: i32,
    pub PersonGuid: Uuid,
    pub FirstName: String,
    pub LastName: String,
    pub Age: i32,
    #[serde(serialize_with = "serialize_iso8601", deserialize_with = "deserialize_iso8601")]
    pub BirthDate: DateTime<Utc>,
    pub Email: String,
    pub PhoneNumber: String,
    pub Address: XAddressInfo,
    pub IsActive: bool,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct XAddressInfo {
    pub Street: String,
    pub Number: String,
    pub Complement: String,
    pub Neighborhood: String,
    pub City: String,
    pub State: String,
    pub PostalCode: String,
    pub Country: String,
    pub Latitude: f64,
    pub Longitude: f64,
}
