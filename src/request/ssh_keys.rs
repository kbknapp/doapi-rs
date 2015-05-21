
use std::fmt;

use response;
use request::{DoRequest, Request};

pub struct SshKeys<'t>(DoRequest<'t>);

impl<'t> SshKeys<'t> {
    pub fn with_token(token: &'t str) -> SshKeys {
        SshKeys( DoRequest::new("https://api.digitalocean.com/v2/account".to_owned(), token) )
    }
}

impl<'t> Request for SshKeys<'t> {
    type RespT = response::SshKey;
    fn auth(&self) -> &str {
        self.0.auth_token
    }
    fn url(&self) -> &str {
        &self.0.url_str[..]
    }
    fn retrieve(&self) -> Result<response::SshKey, String> {
        self.0.retrieve_obj::<response::SshKey>("ssh_key".to_owned())
    }
}

impl<'t> fmt::Display for SshKeys<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}