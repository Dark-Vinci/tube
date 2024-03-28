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
