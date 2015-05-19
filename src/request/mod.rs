pub use self::account::Account;

mod account;
mod action;
mod basic;

use std::io::Read;

use hyper::client;
use hyper::{self, Error, Url};
use hyper::method::Method;
use hyper::net::Fresh;
use hyper::header::{ContentType, Authorization};

use serde::json::{self, Value};
use serde::de::Deserialize;

use response::DoError;

pub trait Request {
    type RespT;
    fn retrieve(&self) -> Result<<Self as Request>::RespT, String>;
    fn url(&self) -> &str;
    fn auth(&self) -> &str;

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

    fn retrieve_obj<T: Deserialize>(&self, obj: String) -> Result<T, String> {
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
}
