use crate::connections::db::DBConnection;
use crate::repository::fruits::FruitsRepo;

#[derive(Debug)]
pub struct Repo {
    pub fruit: FruitsRepo,
}

impl Repo {
    pub fn new(d: DBConnection) -> Self {
        let fr = FruitsRepo::new(d);

        Self {
            fruit: fr,
        }
    }
}
