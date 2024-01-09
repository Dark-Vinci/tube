use crate::connections::db::DBConnection;
use crate::repository::users::UserRepo;

#[derive(Debug)]
pub struct Repo {
    pub user: UserRepo,
}

impl Repo {
    pub fn new(d: DBConnection) -> Self {
        let user = UserRepo::new(d);

        Self {
            user,
        }
    }
}
