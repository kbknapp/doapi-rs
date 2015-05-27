use response;
use request::RequestBuilder;
use request::DoRequest;

pub struct SshKey;

impl<'t> RequestBuilder<'t, response::SshKeys> {
    pub fn create(&self, name: &str, pub_key: &str) -> RequestBuilder<'t, response::SshKey> {
        unimplemented!()
    }
    pub fn update(&self, name: &str, id: &str) -> RequestBuilder<'t, response::SshKey> {
        unimplemented!()
    }
    pub fn destroy(&self, id: &str) -> RequestBuilder<'t, response::SshKey> {
        unimplemented!()
    }
    pub fn show(&self, id: &str, finger: &str) -> RequestBuilder<'t, response::SshKey> {
        unimplemented!()
    }
}

impl<'t> DoRequest<response::SshKey> for RequestBuilder<'t, response::SshKey> {}