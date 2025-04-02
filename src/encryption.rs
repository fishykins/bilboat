pub type Encrypt = fn(message: &str, key: &str) -> String;
pub type Unencrypt = fn(encryption: &str, key: &str) -> String;
