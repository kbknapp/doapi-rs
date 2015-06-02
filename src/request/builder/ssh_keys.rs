use std::marker::PhantomData;
use std::collections::HashMap;

use hyper::method::Method;

use response;
use request::RequestBuilder;
use request::DoRequest;

pub struct SshKey;

impl<'t> RequestBuilder<'t, response::SshKeys> {
    pub fn create(self, name: &str, pub_key: &str) -> RequestBuilder<'t, response::SshKey> {
        // POST: "https://api.digitalocean.com/v2/account/keys"
        // body:
        //      "public_key" : "lkajflasndvioanvinasd"
        //      "name" : "my super key"
        let mut hm = HashMap::new();
        hm.insert("name", name.to_owned());
        hm.insert("public_key", pub_key.to_owned());
        RequestBuilder {
            method: Method::Post,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(hm)
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
        let mut hm = HashMap::new();
        hm.insert("name", name.to_owned());
        RequestBuilder {
            method: Method::Put,
            url: self.url,
            auth: self.auth,
            resp_t: PhantomData,
            body: Some(hm)
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
            body: None
        }
    }

    pub fn show(self) -> RequestBuilder<'t, response::SshKey> {
        // GET: "https://api.digitalocean.com/v2/account/keys/$ID"
        // OR
        // GET: "https://api.digitalocean.com/v2/account/keys/$FINGER"
        self
    }
}

impl<'t> DoRequest<response::SshKey> for RequestBuilder<'t, response::SshKey> {}