use crate::connections::db::DBConnection;
use crate::connections::redis::Redis;

pub struct Connections {
    pub db: DBConnection,
    pub redis: Redis,
}

impl Connections {
    pub fn new(d: DBConnection, r: Redis) -> Self {
        Self {
            db: d,
            redis: r,
        }
    }
}