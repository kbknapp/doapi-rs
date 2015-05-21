use std::fmt;

use response;
use request::{DoRequest, Request};

pub struct Images<'t>(DoRequest<'t>);

impl<'t> Images<'t> {
    pub fn with_token(token: &'t str) -> Images {
        Images( DoRequest::new("https://api.digitalocean.com/v2/images".to_owned(), token) )
    }
}

impl<'t> Request for Images<'t> {
    type RespT = response::Image;
    fn auth(&self) -> &str {
        self.0.auth_token
    }
    fn url(&self) -> &str {
        &self.0.url_str[..]
    }
    fn retrieve(&self) -> Result<response::Image, String> {
        self.0.retrieve_obj::<response::Image>("image".to_owned())
    }
}

impl<'t> fmt::Display for Images<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}