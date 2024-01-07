use tonic::async_trait;

use crate::application::application::App;
use crate::application::traits::SignIn;

#[async_trait]
impl SignIn for App {
    async fn in_with_google(&self) -> () {
        unimplemented!()
    }

    async fn in_with_email(&self) -> () {
        unimplemented!()
    }
}
