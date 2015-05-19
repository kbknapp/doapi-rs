use std::io::Read;
use std::fmt;

use hyper::client::Request;
use hyper::{self, Error, Url};
use hyper::method::Method;
use hyper::net::Fresh;
use hyper::header::{ContentType, Authorization};

use serde::json::{self, Value};

use response;

pub struct Account<'u, 't>(Request<'u, 't>);

impl<'u, 't> Account<'u, 't> {
    pub fn with_token(token: &'t str) -> Account {
        Account( Request::new("https://api.digitalocean.com/v2/account", token) )
    }

    pub fn request(&self) -> hyper::Result<Request<Fresh>> {
        let Account(ref req) = self;
        req.request()
    }

    pub fn retrieve_json(&self) -> hyper::Result<String> {
        let Account(ref req) = self;
        req.retrieve_json()
    }

    pub fn retrieve(&self) -> Result<response::Account, String> {
        let Account(ref req) = self;
        req.retrieve("account")
    }

    pub fn action<'b>(&self, id: &'b str) -> Result<response::Action, String> {
        let req = Request::new(&format!("https://api.digitalocean.com/v2/actions/{}", id), self.auth);
        req.retrieve("action")
    }

    pub fn action_request<'b>(&self, id: &'b str) -> hyper::Result<Request<Fresh>> {
        let req = Request::new(&format!("https://api.digitalocean.com/v2/actions/{}", id), self.auth);
        req.request()
    }

    pub fn retrieve_action_json<'b>(&self, id: &'b str) -> hyper::Result<String> {
        let req = Request::new(&format!("https://api.digitalocean.com/v2/actions/{}", id), self.auth);
        req.retrieve_json()
    }
}

impl<'a> fmt::Display for Account<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "method: GET\n\
                content-type: application/json\n\
                authorization: Bearer {}\n\
                url: {}", self.auth, self.url)
    }
}
