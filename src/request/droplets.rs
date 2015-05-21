
use std::fmt;

use response;
use request::{DoRequest, Request};

pub struct Droplets<'t>(DoRequest<'t>);

impl<'t> Droplets<'t> {
    pub fn with_token(token: &'t str) -> Droplets {
        Droplets( DoRequest::new("https://api.digitalocean.com/v2/droplets".to_owned(), token) )
    }
}

impl<'t> Request for Droplets<'t> {
    type RespT = response::Droplet;
    fn auth(&self) -> &str {
        self.0.auth_token
    }
    fn url(&self) -> &str {
        &self.0.url_str[..]
    }
    fn retrieve(&self) -> Result<response::Droplet, String> {
        self.0.retrieve_obj::<response::Droplet>("droplet".to_owned())
    }
}

impl<'t> fmt::Display for Droplets<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}