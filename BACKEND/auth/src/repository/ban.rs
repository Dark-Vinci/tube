use {
    async_trait::async_trait,
    sdk::models::db::auth::ban::{ActiveModel, Entity as Channel, Model},
    sea_orm::{
        prelude::Uuid, ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait,
        IntoActiveModel,
    },
    std::sync::Arc,
    tracing::{debug, error, Level},
};

#[derive(Debug)]
pub struct BanRepo(Arc<DatabaseConnection>);

impl BanRepo {
    pub fn create(
        d: Arc<DatabaseConnection>,
    ) -> Box<dyn BanRepository + Send + Sync + 'static> {
        Box::new(Self(d))
    }
}

#[async_trait]
pub trait BanRepository {
    async fn create(&self, request_id: Uuid, b: Model) -> Result<Model, String>;
    async fn get_many(&self, request_id: Uuid) -> Result<Vec<Model>, String>;
    async fn get_by_id(&self, request_id: Uuid, id: Uuid) -> Result<Model, String>;
    async fn delete_by_id(&self, request_id: Uuid, id: Uuid) -> Result<bool, String>;
}

#[async_trait]
impl BanRepository for BanRepo {
    #[tracing::instrument(
        name = "BanRepo -> CREATE",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn create(&self, _request_id: Uuid, _b: Model) -> Result<Model, String> {
        debug!("[Got] create ban request");

        let a = ActiveModel {
            ..Default::default()
        };

        let k = a.insert(&*self.0).await;

        if let Err(e) = k {
            error!(error = &e.to_string(), "Failed to create ban");

            return Err(e.to_string());
        }

        Ok(k.unwrap())
    }

    #[tracing::instrument(
        name = "BanRepo -> GET_MANY",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn get_many(&self, _request_id: Uuid) -> Result<Vec<Model>, String> {
        debug!("[Got] get many ban request");

        let v = Channel::find().all(&*self.0).await;

        if let Err(e) = v {
            error!(error = &e.to_string(), "Failed to get many channels");

            return Err(e.to_string());
        }

        return Ok(v.unwrap());
    }

    #[tracing::instrument(
        name = "BanRepo -> GET_BY_ID",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn get_by_id(&self, _request_id: Uuid, _id: Uuid) -> Result<Model, String> {
        debug!("[Got] get ban by id request");

        let res = Channel::find_by_id(Uuid::new_v4()).one(&*self.0).await;

        if let Err(err) = res {
            error!(error = &err.to_string(), "Failed to get ban by id");

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
        name = "BanRepo -> DELETE_BY_ID",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn delete_by_id(&self, _request_id: Uuid, _id: Uuid) -> Result<bool, String> {
        debug!("[Got] delete ban by id request");

        let res = Channel::find_by_id(Uuid::new_v4()).one(&*self.0).await;

        if let Err(err) = res {
            error!(error = &err.to_string(), "Failed to delete ban by id");

            return match err {
                DbErr::RecordNotFound(val) => {
                    let message = format!("{} record not found", val);
                    Err(message)
                },

                _ => Err(err.to_string()),
            };
        }

        let res = res.unwrap().unwrap();

        let a = Channel::delete(res.into_active_model())
            .exec(&*self.0)
            .await;

        if let Err(err) = a {
            error!(error = &err.to_string(), "Failed to delete ban by id");

            return Err(err.to_string());
        }

        return Ok(true);
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
