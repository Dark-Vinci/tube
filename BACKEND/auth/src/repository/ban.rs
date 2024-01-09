use sea_orm::prelude::Uuid;
use sea_orm::{
    ActiveModelTrait,
    DatabaseConnection,
    DbErr, EntityTrait, IntoActiveModel,
};
use tracing_core::Level;
use tracing::{error, debug};

use sdk::models::db::auth::ban::{
    Model,
    ActiveModel,
    Entity as Channel,
};

use crate::connections::db::DBConnection;

#[derive(Debug)]
pub struct BanRepo(DatabaseConnection);

impl BanRepo {
    pub fn new(d: &DBConnection) -> Self {
        let c = d.get_connection().clone();
        Self(c)
    }
}

impl BanRepo {
    #[tracing::instrument(
        name="BanRepo -> CREATE",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    pub async fn create(&self, request_id: Uuid, b: Model) -> Result<Model, String> {
        debug!("[Got] create ban request");

        let a = ActiveModel {
            ..Default::default()
        };

        let k = a.insert(&self.0).await;

        if let Err(e) = k {
            error!(
                error = &e.to_string(),
                "Failed to create ban"
            );

            return Err(e.to_string());
        }

        Ok(k.unwrap())
    }

    #[tracing::instrument(
        name="BanRepo -> GET_MANY",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    pub async fn get_many(&self, request_id: Uuid) -> Result<Vec<Model>, String> {
        debug!("[Got] get many ban request");

        let v = Channel::find()
            .all(&self.0)
            .await;

        if let Err(e) = v {
            error!(
                error = &e.to_string(),
                "Failed to get many channels"
            );

            return Err(e.to_string());
        }

        return Ok(v.unwrap());
    }

    #[tracing::instrument(
        name="BanRepo -> GET_BY_ID",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    pub async fn get_by_id(&self, request_id: Uuid, id: Uuid) -> Result<Model, String> {
        debug!("[Got] get ban by id request");

        let res = Channel::find_by_id(id)
            .one(&self.0)
            .await;

        if let Err(err) = res {
            error!(
                error = &err.to_string(),
                "Failed to get ban by id"
            );
            
            match err {
                DbErr::RecordNotFound(val) => {
                    let message = format!("{} record not found", val);
                    return Err(message)
                },

                _ => return Err(err.to_string())
            }
        }

        let res = res.unwrap().unwrap();

        return Ok(res);
    }

    #[tracing::instrument(
        name="BanRepo -> DELETE_BY_ID",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    pub async fn delete_by_id(&self, request_id: Uuid, id: Uuid) -> Result<bool, String> {
        debug!("[Got] delete ban by id request");

        let res = Channel::find_by_id(id)
            .one(&self.0)
            .await;

        if let Err(err) = res {
            error!(
                error = &err.to_string(),
                "Failed to delete ban by id"
            );

            return match err {
                DbErr::RecordNotFound(val) => {
                    let message = format!("{} record not found", val);
                    Err(message)
                },

                _ => Err(err.to_string())
            }
        }

        let res = res.unwrap().unwrap();

        let a = Channel::delete(res.into_active_model())
            .exec(&self.0)
            .await;

        if let Err(err) = a {
            error!(
                error = &err.to_string(),
                "Failed to delete ban by id"
            );

            return Err(err.to_string());
        }

        return Ok(true);
    }
}
