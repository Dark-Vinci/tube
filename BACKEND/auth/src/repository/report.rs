#[cfg(test)]
use mockall::{automock, predicate::*};
use {
    sdk::models::db::auth::report::{ActiveModel, Entity as Session, Model},
    sea_orm::{
        prelude::Uuid, ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DbErr,
        EntityTrait, IntoActiveModel,
    },
    std::sync::Arc,
    tracing::{debug, error, Level},
};

#[derive(Debug)]
pub struct ReportRepo(Arc<DatabaseConnection>);

impl ReportRepo {
    pub fn create(
        d: Arc<DatabaseConnection>,
    ) -> Box<dyn ReportRepository + Send + Sync + 'static> {
        Box::new(Self(d))
    }
}

#[cfg_attr(test, automock)]
#[async_trait::async_trait]
pub trait ReportRepository {
    async fn create(&self, request_id: Uuid, b: Model) -> Result<Model, String>;
    async fn get_many(&self, request_id: Uuid) -> Result<Vec<Model>, String>;
    async fn get_by_id(&self, request_id: Uuid, id: Uuid) -> Result<Model, String>;
    async fn delete_by_id(&self, request_id: Uuid, id: Uuid) -> Result<bool, String>;
}

#[async_trait::async_trait]
impl ReportRepository for ReportRepo {
    #[tracing::instrument(
        name = "ReportRepo -> CREATE",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn create(&self, _request_id: Uuid, b: Model) -> Result<Model, String> {
        debug!("[Got] create report request");

        let a = ActiveModel {
            created_at: Set(b.created_at),
            ..Default::default()
        };

        let k = a.insert(&*self.0).await;

        if let Err(e) = k {
            error!(error = &e.to_string(), "Failed to create report");

            return Err(e.to_string());
        }

        Ok(k.unwrap())
    }

    #[tracing::instrument(
        name = "ReportRepo -> GET_MANY",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn get_many(&self, _request_id: Uuid) -> Result<Vec<Model>, String> {
        debug!("[Got] get many report request");

        let v = Session::find().all(&*self.0).await;

        if let Err(e) = v {
            error!(error = &e.to_string(), "Failed to get many users");

            return Err(e.to_string());
        }

        return Ok(v.unwrap());
    }

    #[tracing::instrument(
        name = "ReportRepo -> GET_BY_ID",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn get_by_id(&self, _request_id: Uuid, id: Uuid) -> Result<Model, String> {
        debug!("[Got] get report by id request");

        let res = Session::find_by_id(id).one(&*self.0).await;

        if let Err(err) = res {
            error!(error = &err.to_string(), "Failed to get report by id");

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
        name = "ReportRepo -> DELETE_BY_ID",
        skip(self),
        err(level = Level::ERROR),
        level = Level::DEBUG,
        ret,
    )]
    async fn delete_by_id(&self, _request_id: Uuid, id: Uuid) -> Result<bool, String> {
        debug!("[Got] delete report by id request");

        let res = Session::find_by_id(id).one(&*self.0).await;

        if let Err(err) = res {
            error!(error = &err.to_string(), "Failed to delete report by id");

            match err {
                DbErr::RecordNotFound(val) => {
                    let message = format!("{} record not found", val);
                    return Err(message);
                },

                _ => return Err(err.to_string()),
            }
        }

        let res = res.unwrap().unwrap();

        let a = Session::delete(res.into_active_model())
            .exec(&*self.0)
            .await;

        if let Err(err) = a {
            error!(error = &err.to_string(), "Failed to delete report by id");

            return Err(err.to_string());
        }

        return Ok(true);
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[tokio::test]
//     async fn tests() {
//         let mut mock = MockReportRepository::new();

//         mock.expect_create().times(1).returning(|a, b| {
//             Ok(Model {
//                 id: todo!(),
//                 user_id: todo!(),
//                 channel_id: todo!(),
//                 is_active: todo!(),
//                 created_at: todo!(),
//                 updated_at: todo!(),
//                 deleted_at: todo!(),
//                 expires_at: todo!(),
//             })
//         });

//         mock.expect_delete_by_id().times(1).returning(|_| Ok(11));

//         let result1 = mock.function1(110).await;
//         assert_eq!(result1, 10);

//         let result2 = mock.function2(111).await.expect("Got an error");

//         assert_eq!(result2, 11);
//     }
// }
