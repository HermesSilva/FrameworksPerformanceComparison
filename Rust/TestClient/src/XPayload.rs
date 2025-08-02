// Adicione estas dependências ao seu arquivo Cargo.toml:
// chrono = { version = "0.4", features = ["serde"] }
// rust_decimal = "1.35"
// uuid = { version = "1.8", features = ["v4", "serde"] }

use chrono::{DateTime, Duration, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

/// Corresponde à classe XAddressInfo em C#.
#[derive(Debug, Clone, Default)]
pub struct XAddressInfo {
    pub street: String,
    pub number: String,
    pub complement: String,
    pub neighborhood: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
}

/// Corresponde à classe XPersonData em C#.
#[derive(Debug, Clone, Default)]
pub struct XPersonData {
    pub id: i32,
    pub person_guid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub birth_date: DateTime<Utc>,
    pub email: String,
    pub phone_number: String,
    pub address: XAddressInfo,
    pub is_active: bool,
}

/// Corresponde à classe XPayload em C#.
#[derive(Debug, Clone, Default)]
pub struct XPayload {
    // Tipos Inteiros
    pub byte_field: u8,
    pub s_byte_field: i8,
    pub short_field: i16,
    pub u_short_field: u16,
    pub int_field: i32,
    pub u_int_field: u32,
    pub long_field: i64,
    pub u_long_field: u64,

    // Tipos de Ponto Flutuante
    pub float_field: f32,
    pub double_field: f64,
    pub decimal_field: Decimal,

    // Outros Tipos Primitivos
    pub bool_field: bool,
    pub char_field: char,
    pub string_field: String,

    // Tipos Especiais
    pub guid_field: Uuid,
    pub date_time_field: DateTime<Utc>,
    pub time_span_field: Duration,

    // Coleções
    pub persons: Vec<XPersonData>,
}
