use {
    super::{application::App, traits::Short},
    tonic::async_trait,
};

#[async_trait]
impl Short for App {}
