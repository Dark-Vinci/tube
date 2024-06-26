use {
    crate::helpers::db_util::compute_password_hash,
    sea_orm::{entity::prelude::*, ActiveValue},
    serde::{Deserialize, Serialize},
    tonic::async_trait,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users", schema_name = "public")]
pub struct Model {
    #[sea_orm(
        primary_key,
        column_type = "Uuid",
        column_name = "id",
        auto_increment = false
    )]
    pub id: Uuid,

    #[sea_orm(column_type = "Text", nullable)]
    pub first_name: String,

    #[sea_orm(column_type = "Text", nullable)]
    pub last_name: String,

    #[sea_orm(column_type = "DateTime")]
    pub date_of_birth: DateTime,

    #[sea_orm(
        column_type = "Boolean",
        column_name = "is_active",
        default_value = false
    )]
    pub is_active: bool,

    #[sea_orm(column_name = "nick_name")]
    pub nick_name: String,

    #[sea_orm(
        column_type = "DateTime",
        column_name = "created_at",
        default_value = "CURRENT_TIMESTAMP"
    )]
    pub created_at: DateTime,

    #[sea_orm(
        column_type = "DateTime",
        column_name = "updated_at",
        default_value = "CURRENT_TIMESTAMP"
    )]
    pub updated_at: DateTime,

    #[sea_orm(column_type = "DateTime", column_name = "description", nullable)]
    pub deleted_at: Option<DateTime>,

    #[sea_orm(column_name = "password")]
    pub password: String,

    #[sea_orm(column_name = "email", unique, indexed)]
    pub email: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::channel::Entity")]
    Channel,

    #[sea_orm(has_many = "super::session::Entity")]
    Session,

    #[sea_orm(has_many = "super::short::Entity")]
    Short,
}

impl Related<super::channel::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Channel.def()
    }
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl Related<super::short::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Short.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        // is changed and is new
        let is_changed = self.get(Column::Password).is_unchanged();

        if insert || is_changed {
            let password = self.password.into_value().clone().unwrap().to_string();

            let hash = compute_password_hash(password);

            if let Err(err) = hash {
                return Err(DbErr::Custom(err.to_string()));
            }

            self.password = ActiveValue::Set(hash.unwrap());
        }

        Ok(self)
    }
}
