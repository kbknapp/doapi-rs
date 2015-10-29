use std::marker::PhantomData;

use hyper::method::Method;

use response;
use request::RequestBuilder;
use request::DoRequest;

impl<'t> RequestBuilder<'t, response::SshKeys> {
    pub fn create(self, name: &str, pub_key: &str) -> RequestBuilder<'t, response::SshKey> {
        // POST: "https://api.digitalocean.com/v2/account/keys"
        // body:
        //      "public_key" : "lkajflasndvioanvinasd"
        //      "name" : "my super key"
        RequestBuilder {
            method: Method::Post,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(format!("{{\"name\":{:?},\"public_key\":{:?}}}", name, pub_key)),
        }
    }
}

impl<'t> RequestBuilder<'t, response::SshKey> {
    pub fn update(self, name: &str) -> RequestBuilder<'t, response::SshKey> {
        // PUT: "https://api.digitalocean.com/v2/account/keys/$ID"
        // OR
        // PUT: "https://api.digitalocean.com/v2/account/keys/$FINGER"
        // body:
        //      "name" : "new_name"
        RequestBuilder {
            method: Method::Put,
            url: self.url,
            auth: self.auth,
            resp_t: PhantomData,
            body: Some(format!("{{\"name\":{:?}}}", name)),
        }
    }
    pub fn destroy(self) -> RequestBuilder<'t, response::HeaderOnly> {
        // DELETE: "https://api.digitalocean.com/v2/account/keys/$ID"
        // OR
        // DELETE: "https://api.digitalocean.com/v2/account/keys/$FINGER"
        RequestBuilder {
            method: Method::Delete,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: None,
        }
    }
}

impl<'t> DoRequest<response::SshKey> for RequestBuilder<'t, response::SshKey> {}
