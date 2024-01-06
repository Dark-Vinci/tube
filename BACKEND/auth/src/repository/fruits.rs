use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;

use crate::models;
use crate::models::cake::Model;

#[derive(Debug)]
pub struct FruitsRepo;

impl FruitsRepo {
    fn new() -> Self {
        Self
    }
}

impl FruitsRepo {
    async fn create(&self, b: Model) -> Result<Model, String> {
        let a = models::cake::ActiveModel {
            name: Set(b.name),
            ..Default::default()
        };

        let k = a.insert(&3).await;

        if let Err(e) = k {
            Err(e.to_string())
        }

        Ok(k.unwrap())
    }
}
