use std::marker::PhantomData;

use response;
use request::RequestBuilder;
use request::DoRequest;

impl<'t> RequestBuilder<'t, response::Account> {
    pub fn action(self, id: &str) -> RequestBuilder<'t, response::Action> {
        RequestBuilder {
            url: Some(format!("https://api.digitalocean.com/v2/actions/{}", id)),
            auth: self.auth,
            resp_t: PhantomData
        }
    }

    pub fn actions(self) -> RequestBuilder<'t, response::Actions> {
        RequestBuilder {
            url: Some("https://api.digitalocean.com/v2/actions".to_owned()),
            auth: self.auth,
            resp_t: PhantomData
        }
    }
}

impl<'t> DoRequest<response::Account> for RequestBuilder<'t, response::Account> {}