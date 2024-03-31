use {
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

#[derive(Serialize, Deserialize)]
pub struct Empty {}

#[derive(Serialize, Deserialize)]
pub struct Id {
    pub id: Uuid,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Environment {
    Production,
    Development,
    Testing,
}

impl From<&str> for Environment {
    fn from (s: &str) -> Self {
        match s {
            "production" => Self::Production,
            "development" => Self::Development,
            "test" => Self::Testing,
            _ => panic!("Unknown environment"),
        }
    }
}
