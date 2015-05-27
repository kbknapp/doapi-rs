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
    pub fn show(&self, slug: bool) -> RequestBuilder<'t, response::Image> {
        unimplemented!()
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
    pub fn list_actions(&self) -> RequestBuilder<'t, response::Image> {
        unimplemented!()
    }
    pub fn update(&self) -> RequestBuilder<'t, response::Image> {
        unimplemented!()
    }
    pub fn delete(&self) -> RequestBuilder<'t, response::Image> {
        unimplemented!()
    }
    pub fn actions(&self) -> RequestBuilder<'t, response::Actions> {
        unimplemented!()
    }
    pub fn action(&self, id: &str) -> RequestBuilder<'t, response::Action> {
        unimplemented!()
    }
}

impl<'t> DoRequest<response::Image> for RequestBuilder<'t, response::Image> {}

