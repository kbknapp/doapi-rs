pub use self::account::Account;
pub use self::regions::Regions;
pub use self::sizes::Sizes;
pub use self::domains::Domains;
pub use self::droplets::Droplets;
pub use self::images::Images;
pub use self::ssh_keys::SshKeys;
pub use self::dns::Dns;

mod account;
mod action;
mod basic;
mod domains;
mod droplets;
mod images;
mod regions;
mod sizes;
mod ssh_keys;
mod dns;

use std::io::Read;

use hyper::client;
use hyper::{self, Error, Url};
use hyper::method::Method;
use hyper::net::Fresh;
use hyper::header::{ContentType, Authorization};

use serde::json::{self, Value};
use serde::de::Deserialize;

use response::DoError;

pub trait NamedRequest {
    type RespT;
    fn name(&self) -> &str;
}

pub trait Request : NamedRequest {
    fn url(&self) -> &str;
    fn auth(&self) -> &str;
    fn request(&self) -> hyper::Result<client::Request<Fresh>>;
    fn retrieve(&self) -> Result<<Self as DoRequest>::RespT, String>;
    fn retrieve_json(&self) -> hyper::Result<String>;
    fn retrieve_obj(&self, obj: String) -> Result<<Self as DoRequest>::RespT, String>;
}

impl<T> Request for T 
              where T: Deserialize + NamedRequest {

    fn request(&self) -> hyper::Result<client::Request<Fresh>> {
        let url = match Url::parse(self.url()) {
            Ok(url) => url,
            Err(e) => return Err(Error::Uri(e)),
        };
        let mut fresh_req = match client::Request::new(Method::Get, url) {
            Ok(req) => req,
            Err(e)  => return Err(e)
        };
        let mut auth_s = String::new();
        auth_s.push_str("Bearer ");
        auth_s.push_str(self.auth());
        fresh_req.headers_mut().set(ContentType("application/json".parse().unwrap()));
        fresh_req.headers_mut().set(Authorization(auth_s));
        Ok(fresh_req)
    }

    fn retrieve_json(&self) -> hyper::Result<String> {
        let url = match Url::parse(self.url()) {
            Ok(url) => url,
            Err(e) => return Err(Error::Uri(e)),
        };
        let mut fresh_req = match client::Request::new(Method::Get, url) {
            Ok(req) => req,
            Err(e)  => return Err(e)
        };
        let mut auth_s = String::new();
        auth_s.push_str("Bearer ");
        auth_s.push_str(self.auth());
        fresh_req.headers_mut().set(ContentType("application/json".parse().unwrap()));
        fresh_req.headers_mut().set(Authorization(auth_s));
        let streaming_req = try!(fresh_req.start());
        let mut response = try!(streaming_req.send());
        let mut s = String::new();
        try!(response.read_to_string(&mut s));
        Ok(s)
    }

    fn retrieve_obj(&self, obj: String) -> Result<T, String> {
        match self.retrieve_json() {
            Ok(ref s) => {
                match json::from_str::<Value>(s) {
                    Ok(ob) => {
                        match ob.find(&obj) {
                            Some(t) => {
                                match json::from_value(t.clone()) {
                                    Ok(t) => Ok(t),
                                    Err(e) => Err(e.to_string())
                                }
                            },
                            None => {
                                match json::from_value::<DoError>(ob.clone()) {
                                    Ok(err) => Err(err.to_string()),
                                    Err(e)  => Err(e.to_string())
                                }
                            }
                        }
                    },
                    Err(e) => Err(e.to_string())
                }
            },
            Err(e) => Err(e.to_string())
        }
    }

    fn retrieve(&self) -> Result<<Self as DoRequest>::RespT, String> {
        self.retrieve_obj(self.name().to_owned())
    }
}

pub struct DoRequest<'t> {
    pub url_str: String,
    pub auth_token: &'t str
}

impl<'t> DoRequest<'t> {
    pub fn new(url: String, token: &'t str) -> DoRequest<'t> {
        DoRequest {
            url_str: url,
            auth_token: token
        }
    }
}

impl<'t> Request for DoRequest<'t> {
    fn auth(&self) -> &str {
        self.auth_token
    }
    fn url(&self) -> &str {
        &self.url_str[..]
    }
}

impl<'t> fmt::Display for BasicRequest<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "method: GET\n\
                content-type: application/json\n\
                authorization: Bearer {}\n\
                url: {}", self.auth_token, self.url_str)
    }
}