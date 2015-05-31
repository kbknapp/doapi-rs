use std::marker::PhantomData;

use response;
use request::RequestBuilder;
use request::DoRequest;

impl<'t> RequestBuilder<'t, response::Account> {
    pub fn action(mut self, id: &str) -> RequestBuilder<'t, response::Action> {
        // https://api.digitalocean.com/v2/actions/$ID
        RequestBuilder {
            url: format!("https://api.digitalocean.com/v2/actions/{}", id),
            auth: self.auth,
            resp_t: PhantomData
        }
    }

    pub fn actions(self) -> RequestBuilder<'t, response::Actions> {
        // https://api.digitalocean.com/v2/actions
        RequestBuilder {
            url: "https://api.digitalocean.com/v2/actions".to_owned(),
            auth: self.auth,
            resp_t: PhantomData
        }
    }
}

impl<'t> DoRequest<response::Account> for RequestBuilder<'t, response::Account> {}