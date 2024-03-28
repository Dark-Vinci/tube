use crate::connections::{rabbit::Rabbit, redis::Redis};

pub struct Connections {
    pub redis: Redis,
    // pub db: DB,
    pub rabbit: Rabbit,
}

impl Connections {
    pub fn new(a: Redis, b: Rabbit) -> Self {
        Self {
            redis: a,
            rabbit: b,
        }
    }
}
