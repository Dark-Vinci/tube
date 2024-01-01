use crate::application::application::{App, SignUp};

impl SignUp for App {
    async fn up_with_google(&self) -> () {
        unimplemented!()
    }

    async fn up_with_email(&self) -> () {
        unimplemented!()
    }
}
