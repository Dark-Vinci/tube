use {axum::async_trait, uuid::Uuid};

pub trait Account {}
pub trait Auth {}
pub trait Reactions {}
pub trait Posts {}

#[async_trait]
pub trait Search {
    async fn create(&self);
    async fn get_recent(&self) -> Vec<String>;
    async fn get_all(&self) -> Vec<String>;
    async fn delete_all(&self) -> bool;
    async fn delete_one(&self, id: Uuid) -> bool;
    async fn recover(&self) -> bool;
}

#[async_trait]
pub trait TimeLine {
    async fn get_timeline(&self) -> Vec<String>;
    async fn get_short_timeline(&self) -> Vec<String>;
}

pub trait Application: Account + Auth + Posts + Reactions + Search + TimeLine {}
