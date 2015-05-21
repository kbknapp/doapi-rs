use std::fmt;

use response;
use request::{DoRequest, Request};

pub struct Sizes<'t>(DoRequest<'t>);

impl<'t> Sizes<'t> {
    pub fn with_token(token: &'t str) -> Sizes {
        Sizes( DoRequest::new("https://api.digitalocean.com/v2/sizes".to_owned(), token) )
    }
}

impl<'t> Request for Sizes<'t> {
    type RespT = response::Size;
    fn auth(&self) -> &str {
        self.0.auth_token
    }
    fn url(&self) -> &str {
        &self.0.url_str[..]
    }
    fn retrieve(&self) -> Result<response::Size, String> {
        self.0.retrieve_obj::<response::Size>("size".to_owned())
    }
}

impl<'t> fmt::Display for Sizes<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}