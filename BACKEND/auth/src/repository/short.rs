use sea_orm::prelude::Uuid;
use sea_orm::{
    ActiveModelTrait,
    DatabaseConnection,
    EntityTrait,
    DbErr, 
    IntoActiveModel
};
use sea_orm::ActiveValue::Set;
use tracing_core::Level;
use tracing::{error, debug};

use sdk::models::db::auth::short::{
    Model,
    ActiveModel,
    Entity as Short
};

use crate::connections::db::DBConnection;

#[derive(Debug)]
pub struct ShortRepo(DatabaseConnection);

impl ShortRepo {
    pub fn new(d: &DBConnection) -> Self {
        let c = d.get_connection().clone();
        Self(c)
    }
}

impl ShortRepo {
    #[tracing::instrument(
        name="ShortRepo -> CREATE",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    pub async fn create(&self, request_id: Uuid, b: Model) -> Result<Model, String> {
        debug!("[Got] create short request");

        let a = ActiveModel {
            name: Set(b.name),
            ..Default::default()
        };

        let k = a.insert(&self.0).await;

        if let Err(e) = k {
            error!(
                error = &e.to_string(),
                "Failed to create short"
            );

            return Err(e.to_string());
        }

        Ok(k.unwrap())
    }

    #[tracing::instrument(
        name="ShortRepo -> GET_MANY",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    pub async fn get_many(&self, request_id: Uuid) -> Result<Vec<Model>, String> {
        debug!("[Got] get may short request");

        let v = Short::find()
            .all(&self.0)
            .await;

        if let Err(e) = v {
            error!(
                error = &e.to_string(),
                "Failed to get many users"
            );

            return Err(e.to_string());
        }

        return Ok(v.unwrap());
    }

    #[tracing::instrument(
        name="ShortRepo -> GET_BY_ID",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    pub async fn get_by_id(&self, request_id: Uuid, id: Uuid) -> Result<Model, String> {
        debug!("[Got] get short by id request");

        let res = Short::find_by_id(id)
            .one(&self.0)
            .await;

        if let Err(err) = res {
            error!(
                error = &err.to_string(),
                "Failed to get short by id"
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
        name="ShortRepo -> DELETE_BY_ID",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    pub async fn delete_by_id(&self, request_id: Uuid, id: Uuid) -> Result<bool, String> {
        debug!("[Got] delete short by id request");

        let res = Short::find_by_id(id)
            .one(&self.0)
            .await;

        if let Err(err) = res {
            error!(
                error = &err.to_string(),
                "Failed to delete short by id"
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

        let a = Short::delete(res.into_active_model())
            .exec(&self.0)
            .await;

        if let Err(err) = a {
            error!(
                error = &err.to_string(),
                "Failed to delete short by id"
            );

            return Err(err.to_string());
        }

        return Ok(true);
    }
}
