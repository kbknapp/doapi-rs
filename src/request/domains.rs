
use std::fmt;

use response;
use request::{DoRequest, Request};

pub struct Domains<'t>(DoRequest<'t>);

impl<'t> Domains<'t> {
    pub fn with_token(token: &'t str) -> Domains {
        Domains( DoRequest::new("https://api.digitalocean.com/v2/domains".to_owned(), token) )
    }
}

impl<'t> Request for Domains<'t> {
    type RespT = response::Domain;
    fn auth(&self) -> &str {
        self.0.auth_token
    }
    fn url(&self) -> &str {
        &self.0.url_str[..]
    }
    fn retrieve(&self) -> Result<response::Domain, String> {
        self.0.retrieve_obj::<response::Domain>("domains".to_owned())
    }
}

impl<'t> fmt::Display for Domains<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}