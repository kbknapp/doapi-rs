use std::marker::PhantomData;

use response;
use request::RequestBuilder;
use request::DoRequest;

impl<'t> RequestBuilder<'t, response::Account> {
    pub fn action(self, id: &str) -> RequestBuilder<'t, response::Action> {
        // https://api.digitalocean.com/v2/actions/$ID
        RequestBuilder::new(self.auth, format!("https://api.digitalocean.com/v2/actions/{}", id))
    }

    pub fn actions(self) -> RequestBuilder<'t, response::Actions> {
        // https://api.digitalocean.com/v2/actions
        RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/actions")
    }
}

impl<'t> DoRequest<response::Account> for RequestBuilder<'t, response::Account> {}
