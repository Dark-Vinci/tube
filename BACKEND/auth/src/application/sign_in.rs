use crate::application::application::App;
use crate::application::traits::SignIn;

impl SignIn for App {
    async fn in_with_google(&self) -> () {
        unimplemented!()
    }

    async fn in_with_email(&self) -> () {
        unimplemented!()
    }
}
