use sea_orm::prelude::Uuid;
use sea_orm::{
    ActiveModelTrait,
    DatabaseConnection,
    EntityTrait,
    DbErr
};
use sea_orm::ActiveValue::Set;

use sdk::models::db::auth::user::{
    Model,
    ActiveModel,
    Entity as User
};

use crate::connections::db::DBConnection;

#[derive(Debug)]
pub struct UserRepo(DatabaseConnection);

impl UserRepo {
    pub fn new(d: DBConnection) -> Self {
        let c = d.get_connection().clone();
        Self(c)
    }
}

impl UserRepo {
    pub async fn create(&self, b: Model) -> Result<Model, String> {
        let a = ActiveModel {
            first_name: Set(b.first_name),
            ..Default::default()
        };

        let k = a.insert(&self.0).await;

        if let Err(e) = k {
            return Err(e.to_string());
        }

        Ok(k.unwrap())
    }

    pub async fn get_many(&self) -> Result<Vec<Model>, String> {
        let v = User::find()
            .all(&self.0)
            .await;

        if let Err(e) = v {
            return Err(e.to_string());
        }

        return Ok(v.unwrap());
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Model, String> {
        let res = User::find_by_id(id)
        .one(&self.0)
        .await;

        if let Err(err) = res {
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
}
