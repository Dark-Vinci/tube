use sdk::models::db::auth::user::{
    ActiveModel, Entity as User, Model,
};
use sea_orm::prelude::Uuid;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait,
    IntoActiveModel,
};
use tracing::{debug, error};

use crate::connections::db::DBConnection;

#[derive(Debug)]
pub struct UserRepo<'a>(&'a DatabaseConnection);

impl<'a> UserRepo<'a> {
    pub fn new(d: &DBConnection) -> Self {
        let c = d.get_connection();
        Self(c)
    }
}

impl UserRepo {
    #[tracing::instrument(
    name = "UserRepo -> CREATE",
    skip(self),
    err(level = Level::ERROR),
    level = Level::DEBUG,
    ret,
    )]
    pub async fn create(
        &self,
        request_id: Uuid,
        b: Model,
    ) -> Result<Model, String> {
        debug!("[Got] creae user request");

        let a = ActiveModel {
            first_name: Set(b.first_name),
            ..Default::default()
        };

        let k = a.insert(&self.0).await;

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
    pub async fn get_many(
        &self,
        request_id: Uuid,
    ) -> Result<Vec<Model>, String> {
        debug!("[Got] get many user request");

        let v = User::find().all(&self.0).await;

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
    name = "UserRepo -> GET_BY_ID",
    skip(self),
    err(level = Level::ERROR),
    level = Level::DEBUG,
    ret,
    )]
    pub async fn get_by_id(
        &self,
        request_id: Uuid,
        id: Uuid,
    ) -> Result<Model, String> {
        debug!("[Got] get user by id request");

        let res = User::find_by_id(id).one(&self.0).await;

        if let Err(err) = res {
            error!(
                error = &err.to_string(),
                "Failed to get user by id"
            );

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
    pub async fn delete_by_id(
        &self,
        request_id: Uuid,
        id: Uuid,
    ) -> Result<bool, String> {
        debug!("[Got] delete user by id request");

        let res = User::find_by_id(id).one(&self.0).await;

        if let Err(err) = res {
            error!(
                error = &err.to_string(),
                "Failed to delete user by id"
            );

            match err {
                DbErr::RecordNotFound(val) => {
                    let message = format!("{} record not found", val);
                    return Err(message);
                },

                _ => return Err(err.to_string()),
            }
        }

        let res = res.unwrap().unwrap();

        let a =
            User::delete(res.into_active_model()).exec(&self.0).await;

        if let Err(err) = a {
            error!(
                error = &err.to_string(),
                "Failed to delete user by id"
            );

            return Err(err.to_string());
        }

        return Ok(true);
    }
}
