use std::sync::atomic::{AtomicI32, Ordering};
use std::time::{SystemTime};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::XPayload::{XAddressInfo, XPayload, XPersonData};

pub struct XPayloadGenerator;

static PERSON_ID_COUNTER: AtomicI32 = AtomicI32::new(1);

impl XPayloadGenerator {
    pub fn create_3kb_payload() -> XPayload {
        let mut rng = StdRng::seed_from_u64(42);

        let payload = XPayload {
            ByteField: rng.random::<u8>(),
            SByteField: rng.random::<i8>(),
            ShortField: rng.random::<i16>(),
            UShortField: rng.random::<u16>(),
            IntField: rng.random::<i32>(),
            UIntField: rng.random::<u32>(),
            LongField: rng.random::<i64>(),
            ULongField: rng.random::<u64>() >> 1,

            FloatField: rng.random::<f32>() * 1000.0,
            DoubleField: rng.random::<f64>() * 10000.0,
            DecimalField: (rng.random::<f64>() * 100000.0) as f64,

            BoolField: rng.random::<bool>(),

            CharField: XPayloadGenerator::generate_random_char(&mut rng),
            StringField: XPayloadGenerator::generate_random_string(&mut rng, 200),

            GuidField: uuid::Uuid::new_v4(),

            // DateTimeField: SystemTime::now().into(),
            // TimeSpanField: Duration::from_secs(rng.random_range(0..1440*60)),

            Persons: XPayloadGenerator::generate_person_list(&mut rng, 8),
        };

        payload
    }

    fn generate_person_list(rng: &mut StdRng, count: usize) -> Vec<XPersonData> {
        let mut persons = Vec::with_capacity(count);

        for _ in 0..count {
            let id = PERSON_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
             let birth_date = SystemTime::now().into();
            persons.push(XPersonData {
                Id: id,
                PersonGuid: uuid::Uuid::new_v4(),
                FirstName: XPayloadGenerator::generate_first_name(rng),
                LastName: XPayloadGenerator::generate_last_name(rng),
                Age: rng.random_range(18..80),
                 BirthDate: birth_date,
                Email: XPayloadGenerator::generate_email(rng),
                PhoneNumber: XPayloadGenerator::generate_phone_number(rng),
                Address: XPayloadGenerator::generate_address(rng),
                IsActive: rng.random_range(0..100) < 95,
            });
        }

        persons
    }

    fn generate_address(rng: &mut StdRng) -> XAddressInfo {
        XAddressInfo {
            Street: XPayloadGenerator::generate_street_name(rng),
            Number: rng.random_range(1..5000).to_string(),
            Complement: if rng.random_range(0..3) == 0 {
                XPayloadGenerator::generate_random_string(rng, 10)
            } else {
                String::new()
            },
            Neighborhood: XPayloadGenerator::generate_neighborhood(rng),
            City: XPayloadGenerator::generate_city(rng),
            State: XPayloadGenerator::generate_state(rng),
            PostalCode: XPayloadGenerator::generate_postal_code(rng),
            Country: "Brasil".to_string(),
            Latitude: rng.random_range(-90.0..90.0),
            Longitude: rng.random_range(-180.0..180.0),
        }
    }

    fn generate_random_string(rng: &mut StdRng, length: usize) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789 ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        (0..length)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    fn generate_random_char(rng: &mut StdRng) -> char {
        (rng.random_range(65..91) as u8) as char
    }

    fn generate_first_name(rng: &mut StdRng) -> String {
        let list = vec!["Maria", "João", "Ana", "Pedro", "Carlos", "Fernanda", "Roberto", "Juliana", "Marcos", "Patrícia"];
        list[rng.random_range(0..list.len())].to_string()
    }

    fn generate_last_name(rng: &mut StdRng) -> String {
        let list = vec!["Silva", "Santos", "Oliveira", "Souza", "Pereira", "Costa"];
        list[rng.random_range(0..list.len())].to_string()
    }

    fn generate_email(rng: &mut StdRng) -> String {
        let domains = vec!["gmail.com", "hotmail.com", "yahoo.com", "outlook.com", "empresa.com.br"];
        format!(
            "{}.{}{}@{}",
            XPayloadGenerator::generate_first_name(rng).to_lowercase(),
            XPayloadGenerator::generate_last_name(rng).to_lowercase(),
            rng.random_range(1..999),
            domains[rng.random_range(0..domains.len())]
        )
    }

    fn generate_phone_number(rng: &mut StdRng) -> String {
        format!("({}) 9{}-{}", rng.random_range(11..99), rng.random_range(1000..9999), rng.random_range(1000..9999))
    }

    fn generate_street_name(rng: &mut StdRng) -> String {
        let streets = vec!["Rua das Flores", "Avenida Brasil", "Rua dos Andradas"];
        streets[rng.random_range(0..streets.len())].to_string()
    }

    fn generate_neighborhood(rng: &mut StdRng) -> String {
        let list = vec!["Centro", "Jardins", "Moema", "Vila Madalena"];
        list[rng.random_range(0..list.len())].to_string()
    }

    fn generate_city(rng: &mut StdRng) -> String {
        let list = vec!["São Paulo", "Rio de Janeiro", "Belo Horizonte"];
        list[rng.random_range(0..list.len())].to_string()
    }

    fn generate_state(rng: &mut StdRng) -> String {
        let list = vec!["SP", "RJ", "MG"];
        list[rng.random_range(0..list.len())].to_string()
    }

    fn generate_postal_code(rng: &mut StdRng) -> String {
        format!("{}-{}", rng.random_range(10000..99999), rng.random_range(100..999))
    }

    pub fn create_multiple_payloads(count: usize) -> Vec<XPayload> {
        (0..count).map(|_| Self::create_3kb_payload()).collect()
    }
}
