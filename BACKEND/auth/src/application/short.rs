use tonic::async_trait;

use super::{application::App, traits::Short};


#[async_trait]
impl Short for App {}
