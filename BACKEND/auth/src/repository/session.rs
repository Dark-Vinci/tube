use {
    sdk::models::db::auth::session::{ActiveModel, Entity as Session, Model},
    sea_orm::{
        prelude::Uuid, ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DbErr,
        EntityTrait, IntoActiveModel,
    },
    std::sync::Arc,
    tracing::{debug, error, Level},
};

#[derive(Debug)]
pub struct SessionRepo(Arc<DatabaseConnection>);

impl SessionRepo {
    pub fn create(
        d: Arc<DatabaseConnection>,
    ) -> Box<dyn SessionRepository + Send + Sync + 'static> {
        Box::new(Self(d))
    }
}

#[async_trait::async_trait]
pub trait SessionRepository {
    async fn create(&self, request_id: Uuid, b: Model) -> Result<Model, String>;
    async fn get_many(&self, request_id: Uuid) -> Result<Vec<Model>, String>;
    async fn get_by_id(&self, request_id: Uuid, id: Uuid) -> Result<Model, String>;
    async fn delete_by_id(&self, request_id: Uuid, id: Uuid) -> Result<bool, String>;
}

#[async_trait::async_trait]
impl SessionRepository for SessionRepo {
    #[tracing::instrument(
        name = "SessionRepo -> CREATE",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn create(&self, _request_id: Uuid, b: Model) -> Result<Model, String> {
        debug!("[Got] create session request");

        let a = ActiveModel {
            created_at: Set(b.created_at),
            ..Default::default()
        };

        let k = a.insert(&*self.0).await;

        if let Err(e) = k {
            error!(error = &e.to_string(), "Failed to create session");

            return Err(e.to_string());
        }

        Ok(k.unwrap())
    }

    #[tracing::instrument(
        name = "SessionRepo -> GET_MANY",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn get_many(&self, _request_id: Uuid) -> Result<Vec<Model>, String> {
        debug!("[Got] get many session request");

        let v = Session::find().all(&*self.0).await;

        if let Err(e) = v {
            error!(error = &e.to_string(), "Failed to get many users");

            return Err(e.to_string());
        }

        return Ok(v.unwrap());
    }

    #[tracing::instrument(
        name = "SessionRepo -> GET_BY_ID",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn get_by_id(&self, _request_id: Uuid, id: Uuid) -> Result<Model, String> {
        debug!("[Got] get session by id request");

        let res = Session::find_by_id(id).one(&*self.0).await;

        if let Err(err) = res {
            error!(error = &err.to_string(), "Failed to get session by id");

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
        name = "SessionRepo -> DELETE_BY_ID",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn delete_by_id(&self, _request_id: Uuid, id: Uuid) -> Result<bool, String> {
        debug!("[Got] delete session by id request");

        let res = Session::find_by_id(id).one(&*self.0).await;

        if let Err(err) = res {
            error!(error = &err.to_string(), "Failed to delete session by id");

            return match err {
                DbErr::RecordNotFound(val) => {
                    let message = format!("{} record not found", val);
                    Err(message)
                },

                _ => Err(err.to_string()),
            };
        }

        let res = res.unwrap().unwrap();

        let a = Session::delete(res.into_active_model())
            .exec(&*self.0)
            .await;

        if let Err(err) = a {
            error!(error = &err.to_string(), "Failed to delete session by id");

            return Err(err.to_string());
        }

        return Ok(true);
    }
}

#[cfg(test)]
mod test {}
