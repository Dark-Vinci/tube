use {
    sdk::models::db::auth::channel::{ActiveModel, Entity as Channel, Model},
    sea_orm::{
        prelude::Uuid, ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DbErr,
        EntityTrait, IntoActiveModel,
    },
    std::sync::Arc,
    tracing::{debug, error, Level},
};

#[derive(Debug)]
pub struct ChannelRepo(Arc<DatabaseConnection>);

impl ChannelRepo {
    pub fn new(
        d: Arc<DatabaseConnection>,
    ) -> Box<dyn ChannelRepository + Send + Sync + 'static> {
        Box::new(Self(d))
    }
}

#[async_trait::async_trait]
pub trait ChannelRepository {
    async fn create(&self, request_id: Uuid, b: Model) -> Result<Model, String>;
    async fn get_many(&self, request_id: Uuid) -> Result<Vec<Model>, String>;
    async fn get_by_id(&self, request_id: Uuid, id: Uuid) -> Result<Model, String>;
    async fn delete_by_id(&self, request_id: Uuid, id: Uuid) -> Result<bool, String>;
}

#[async_trait::async_trait]
impl ChannelRepository for ChannelRepo {
    #[tracing::instrument(
        name = "ChannelRepo -> CREATE",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn create(&self, request_id: Uuid, b: Model) -> Result<Model, String> {
        debug!("[Got] creat channel request");

        let a = ActiveModel {
            name: Set(b.name),
            ..Default::default()
        };

        let k = a.insert(&*self.0).await;

        if let Err(e) = k {
            error!(error = &e.to_string(), "Failed to create channel");

            return Err(e.to_string());
        }

        Ok(k.unwrap())
    }

    #[tracing::instrument(
        name = "ChannelRepo -> GET_MANY",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn get_many(&self, request_id: Uuid) -> Result<Vec<Model>, String> {
        debug!("[Got] get many channel request");

        let v = Channel::find().all(&*self.0).await;

        if let Err(e) = v {
            error!(error = &e.to_string(), "Failed to get many channels");

            return Err(e.to_string());
        }

        return Ok(v.unwrap());
    }

    #[tracing::instrument(
        name = "ChannelRepo -> GET_BY_ID",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn get_by_id(&self, request_id: Uuid, id: Uuid) -> Result<Model, String> {
        debug!("[Got] get channel by id request");

        let res = Channel::find_by_id(id).one(&*self.0).await;

        if let Err(err) = res {
            error!(error = &err.to_string(), "Failed to get channel by id");

            match err {
                DbErr::RecordNotFound(val) => {
                    let message = format!("{} record not found", val);
                    return Err(message);
                },

                _ => return Err(err.to_string()),
            }
        }

        let res = res.unwrap().unwrap();

        return Ok(res);
    }

    #[tracing::instrument(
        name = "ChannelRepo -> DELETE_BY_ID",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn delete_by_id(&self, request_id: Uuid, id: Uuid) -> Result<bool, String> {
        debug!("[Got] delete channel by id request");

        let res = Channel::find_by_id(id).one(&*self.0).await;

        if let Err(err) = res {
            error!(error = &err.to_string(), "Failed to delete channel by id");

            match err {
                DbErr::RecordNotFound(val) => {
                    let message = format!("{} record not found", val);
                    return Err(message);
                },

                _ => return Err(err.to_string()),
            }
        }

        let res = res.unwrap().unwrap();

        let a = Channel::delete(res.into_active_model())
            .exec(&*self.0)
            .await;

        if let Err(err) = a {
            error!(error = &err.to_string(), "Failed to delete channel by id");

            return Err(err.to_string());
        }

        return Ok(true);
    }
}
