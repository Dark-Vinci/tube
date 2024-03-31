use {
    sdk::models::db::auth::user::{ActiveModel, Entity as User, Model},
    sea_orm::{
        prelude::Uuid, ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DbErr,
        EntityTrait, IntoActiveModel,
    },
    std::sync::Arc,
    tracing::{debug, error, Level},
};

#[derive(Debug)]
pub struct UserRepo(Arc<DatabaseConnection>);

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create(&self, request_id: Uuid, b: Model) -> Result<Model, String>;
    async fn get_many(&self, request_id: Uuid) -> Result<Vec<Model>, String>;
    async fn get_by_id(&self, request_id: Uuid, id: Uuid) -> Result<Model, String>;
    async fn delete_by_id(&self, request_id: Uuid, id: Uuid) -> Result<bool, String>;
}

impl UserRepo {
    pub fn new(
        d: Arc<DatabaseConnection>,
    ) -> Box<dyn UserRepository + Send + Sync + 'static> {
        Box::new(Self(d))
    }
}

#[async_trait::async_trait]
impl UserRepository for UserRepo {
    #[tracing::instrument(
        name = "UserRepo -> CREATE",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn create(&self, _request_id: Uuid, b: Model) -> Result<Model, String> {
        debug!("[Got] creae user request");

        let a = ActiveModel {
            first_name: Set(b.first_name),
            ..Default::default()
        };

        let k = a.insert(&*self.0).await;

        if let Err(e) = k {
            error!(error = &e.to_string(), "Failed to create user");

            return Err(e.to_string());
        }

        Ok(k.unwrap())
    }

    #[tracing::instrument(
        name = "UserRepo -> GET_MANY",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn get_many(&self, _request_id: Uuid) -> Result<Vec<Model>, String> {
        debug!("[Got] get many user request");

        let v = User::find().all(&*self.0).await;

        if let Err(e) = v {
            error!(error = &e.to_string(), "Failed to get many users");

            return Err(e.to_string());
        }

        return Ok(v.unwrap());
    }

    #[tracing::instrument(
        name = "UserRepo -> GET_BY_ID",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn get_by_id(&self, _request_id: Uuid, id: Uuid) -> Result<Model, String> {
        debug!("[Got] get user by id request");

        let res = User::find_by_id(id).one(&*self.0).await;

        if let Err(err) = res {
            error!(error = &err.to_string(), "Failed to get user by id");

            return match err {
                DbErr::RecordNotFound(val) => {
                    let message = format!("{} record not found", val);
                    Err(message)
                },

                _ => Err(err.to_string()),
            };
        }

        let res = res.unwrap().unwrap();

        return Ok(res);
    }

    #[tracing::instrument(
        name = "UserRepo -> DELETE_BY_ID",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn delete_by_id(&self, _request_id: Uuid, id: Uuid) -> Result<bool, String> {
        debug!("[Got] delete user by id request");

        let res = User::find_by_id(id).one(&*self.0).await;

        if let Err(err) = res {
            error!(error = &err.to_string(), "Failed to delete user by id");

            match err {
                DbErr::RecordNotFound(val) => {
                    let message = format!("{} record not found", val);
                    return Err(message);
                },

                _ => return Err(err.to_string()),
            }
        }

        let res = res.unwrap().unwrap();

        let a = User::delete(res.into_active_model()).exec(&*self.0).await;

        if let Err(err) = a {
            error!(error = &err.to_string(), "Failed to delete user by id");

            return Err(err.to_string());
        }

        return Ok(true);
    }
}
