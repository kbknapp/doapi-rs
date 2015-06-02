use std::marker::PhantomData;
use std::collections::HashMap;

use hyper::method::Method;

use response;
use request::RequestBuilder;
use request::DoRequest;

impl<'t> RequestBuilder<'t, response::Image> {
    pub fn transfer(self, region: &str) -> RequestBuilder<'t, response::Action> {
        // POST: "https://api.digitalocean.com/v2/images/$ID/actions"
        // body:
        //      "type" : "transfer"
        //      "name" : "new_name"
        let mut hm = HashMap::new();
        hm.insert("type", "transfer".to_owned());
        hm.insert("region", region.to_owned());
        RequestBuilder {
            method: Method::Post,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(hm)
        }
    }
    pub fn convert(self) -> RequestBuilder<'t, response::Action> {
        // POST: "https://api.digitalocean.com/v2/images/7938291/actions"
        // body:
        //      "type" : "transfer"
        let mut hm = HashMap::new();
        hm.insert("type", "convert".to_owned());
        RequestBuilder {
            method: Method::Post,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: Some(hm)
        }
    }
    pub fn show(mut self, slug: &str) -> RequestBuilder<'t, response::Image> {
        // GET: https://api.digitalocean.com/v2/images/$SLUG
        self.url.push('/');
        self.url.push_str(slug);
        self
    }
    pub fn actions(mut self) -> RequestBuilder<'t, response::Actions> {
        // GET: https://api.digitalocean.com/v2/images/$SLUG
        self.url.push_str("/actions");
        RequestBuilder {
            url: self.url,
            auth: self.auth,
            method: self.method,
            resp_t: PhantomData,
            body: None
        }
    }
    pub fn action(mut self, id: &str) -> RequestBuilder<'t, response::Action> {
        // GET: https://api.digitalocean.com/v2/images/$IMG_ID/actions/$ID
        self.url.push_str("/actions/");
        self.url.push_str(id);
        RequestBuilder::new(self.auth, self.url)
    }

    pub fn update(mut self, name: &str) -> RequestBuilder<'t, response::Image> {
        // PUT: https://api.digitalocean.com/v2/images/$ID
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
    pub fn delete(self) -> RequestBuilder<'t, response::HeaderOnly> {
        // DELETE: https://api.digitalocean.com/v2/images/$ID
        RequestBuilder {
            method: Method::Delete,
            auth: self.auth,
            url: self.url,
            resp_t: PhantomData,
            body: None
        }
    }
}

impl<'t> RequestBuilder<'t, response::Images> {
    pub fn applications(mut self) -> RequestBuilder<'t, response::Images> {
        // GET: https://api.digitalocean.com/v2/images?type=applications
        self.url.push_str("?type=applications");
        self
    }
    pub fn distributions(mut self) -> RequestBuilder<'t, response::Images> {
        // GET: https://api.digitalocean.com/v2/images?type=distribtutions
        self.url.push_str("?type=distributions");
        self
    }
    pub fn private(mut self) -> RequestBuilder<'t, response::Images> {
        // GET: https://api.digitalocean.com/v2/images?type=private
        self.url.push_str("?type=private");
        self
    }
    pub fn available(mut self) -> RequestBuilder<'t, response::Images> {
        // GET: https://api.digitalocean.com/v2/images?type=available
        self.url.push_str("?type=available");
        self
    }

}

impl<'t> DoRequest<response::Image> for RequestBuilder<'t, response::Image> {}
