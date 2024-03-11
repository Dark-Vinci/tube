pub struct Connections {
    pub redis: Redis,
    pub db: DB,
    pub rabbit: Rabbit,
}

impl Connections {
    pub fn new() -> Self {
        Self {}
    }
}
