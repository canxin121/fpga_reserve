use std::sync::LazyLock;
pub mod db;

pub static STUDENT_ENCODE_KEY: LazyLock<jsonwebtoken::EncodingKey> = LazyLock::new(|| {
    jsonwebtoken::EncodingKey::from_secret(include_bytes!("../secrets/student_key.pkcs8.der"))
});

pub static STUDENT_DECODE_KEY: LazyLock<jsonwebtoken::DecodingKey> = LazyLock::new(|| {
    jsonwebtoken::DecodingKey::from_secret(include_bytes!("../secrets/student_key.pkcs8.der"))
});

pub static TEACHER_ENCODE_KEY: LazyLock<jsonwebtoken::EncodingKey> = LazyLock::new(|| {
    jsonwebtoken::EncodingKey::from_secret(include_bytes!("../secrets/teacher_key.pkcs8.der"))
});

pub static TEACHER_DECODE_KEY: LazyLock<jsonwebtoken::DecodingKey> = LazyLock::new(|| {
    jsonwebtoken::DecodingKey::from_secret(include_bytes!("../secrets/teacher_key.pkcs8.der"))
});
