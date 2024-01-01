use async_trait::async_trait;

use crate::application::application::App;
use crate::application::traits::SignUp;

#[async_trait]
impl SignUp for App {
    async fn up_with_google(&self) -> () {
        unimplemented!()
    }

    async fn up_with_email(&self) -> () {
        unimplemented!()
    }
}
