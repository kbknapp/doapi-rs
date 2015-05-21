use std::fmt;

use response;
use request::{DoRequest, Request};

pub struct Regions<'t>(DoRequest<'t>);

impl<'t> Regions<'t> {
    pub fn with_token(token: &'t str) -> Regions {
        Regions( DoRequest::new("https://api.digitalocean.com/v2/regions".to_owned(), token) )
    }
}

impl<'t> Request for Regions<'t> {
    type RespT = response::Region;
    fn auth(&self) -> &str {
        self.0.auth_token
    }
    fn url(&self) -> &str {
        &self.0.url_str[..]
    }
    fn retrieve(&self) -> Result<response::Region, String> {
        self.0.retrieve_obj::<response::Region>("region".to_owned())
    }
}

impl<'t> fmt::Display for Regions<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}