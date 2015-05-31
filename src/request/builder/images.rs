use std::marker::PhantomData;

use response;
use request::RequestBuilder;
use request::DoRequest;

impl<'t> RequestBuilder<'t, response::Image> {
    pub fn transfer(&self, region: &str) -> RequestBuilder<'t, response::Image> {
        unimplemented!()
    }
    pub fn convert(&self) -> RequestBuilder<'t, response::Image> {
        unimplemented!()
    }
    pub fn show(mut self, slug: &str) -> RequestBuilder<'t, response::Image> {
        // https://api.digitalocean.com/v2/images/$SLUG
        self.url.push('/');
        self.url.push_str(slug);
        RequestBuilder {
            url: self.url,
            auth: self.auth,
            resp_t: PhantomData
        }
    }
    pub fn list_distros(&self) -> RequestBuilder<'t, response::Image> {
        unimplemented!()
    }
    pub fn list_applications(&self) -> RequestBuilder<'t, response::Image> {
        unimplemented!()
    }
    pub fn list_user_images(&self) -> RequestBuilder<'t, response::Image> {
        unimplemented!()
    }
    pub fn actions(mut self) -> RequestBuilder<'t, response::Actions> {
        self.url.push_str("/actions");
        RequestBuilder {
            url: self.url,
            auth: self.auth,
            resp_t: PhantomData
        }
    }
    pub fn update(&self) -> RequestBuilder<'t, response::Image> {
        unimplemented!()
    }
    pub fn delete(&self) -> RequestBuilder<'t, response::Image> {
        unimplemented!()
    }
    pub fn action(&self, id: &str) -> RequestBuilder<'t, response::Action> {
        unimplemented!()
    }
}

impl<'t> RequestBuilder<'t, response::Images> {
    pub fn applications(mut self) -> RequestBuilder<'t, response::Images> {
        self.url.push_str("?type=applications");
        RequestBuilder {
            url: self.url,
            auth: self.auth,
            resp_t: PhantomData
        }
    }
    pub fn distributions(mut self) -> RequestBuilder<'t, response::Images> {
        self.url.push_str("?type=distributions");
        RequestBuilder {
            url: self.url,
            auth: self.auth,
            resp_t: PhantomData
        }
    }
    pub fn private(mut self) -> RequestBuilder<'t, response::Images> {
        self.url.push_str("?type=private");
        RequestBuilder {
            url: self.url,
            auth: self.auth,
            resp_t: PhantomData
        }
    }
    pub fn available(mut self) -> RequestBuilder<'t, response::Images> {
        self.url.push_str("?type=available");
        RequestBuilder {
            url: self.url,
            auth: self.auth,
            resp_t: PhantomData
        }
    }

}

impl<'t> DoRequest<response::Image> for RequestBuilder<'t, response::Image> {}