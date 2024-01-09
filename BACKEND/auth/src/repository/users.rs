use sea_orm::{
    ActiveModelTrait,
    ColumnTrait,
    DatabaseConnection,
    EntityTrait,
    QueryFilter
};
use sea_orm::ActiveValue::Set;

use sdk::models::db::auth::user::{
    Model,
    ActiveModel,
    Entity as User,
    self
};

use crate::connections::db::DBConnection;

#[derive(Debug)]
pub struct FruitsRepo(DatabaseConnection);

impl FruitsRepo {
    pub fn new(d: DBConnection) -> Self {
        let c = d.get_connection().clone();
        Self(c)
    }
}

impl FruitsRepo {
    pub async fn create(&self, b: Model) -> Result<Model, String> {
        let a = ActiveModel {
            name: Set(b.name),
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
            .filter(user::Column::Name.contains("me"))
            .all(&self.0)
            .await;

        if let Err(e) = v {
            return Err(e.to_string());
        }

        return Ok(v.unwrap());
    }
}
