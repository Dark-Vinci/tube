use crate::application::application::App;
use crate::application::traits::SignUp;

impl SignUp for App {
    async fn up_with_google(&self) -> () {
        unimplemented!()
    }

    async fn up_with_email(&self) -> () {
        unimplemented!()
    }
}
