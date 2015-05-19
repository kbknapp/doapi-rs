
use std::io::Read;
use std::fmt;

use hyper::client::Request;
use hyper::{self, Error, Url};
use hyper::method::Method;
use hyper::net::Fresh;
use hyper::header::{ContentType, Authorization};

use serde::json::{self, Value};

use response;

pub struct Action<'a> {
    request: Request<'a>
}

impl<'a> Action<'a> {
    pub fn with_token(t: &'a str) -> Account {
        Account {
            url: "https://api.digitalocean.com/v2/actions",
            auth: t
        }
    }

    pub fn request<'b>(&self, id: &'b str) -> hyper::Result<Request<Fresh>> {
        let url = match Url::parse(format!("{}/{}", self.url, id))) {
            Ok(url) => url,
            Err(e) => return Err(Error::Uri(e)),
        };
        let mut fresh_req = match Request::new(Method::Get, url) {
            Ok(req) => req,
            Err(e)  => return Err(e)
        };
        let mut auth_s = String::new();
        auth_s.push_str("Bearer ");
        auth_s.push_str(self.auth);
        fresh_req.headers_mut().set(ContentType("application/json".parse().unwrap()));
        fresh_req.headers_mut().set(Authorization(auth_s));
        Ok(fresh_req)
    }

    pub fn retrieve_json(&self) -> hyper::Result<String> {
        let url = match Url::parse(self.url) {
            Ok(url) => url,
            Err(e) => return Err(Error::Uri(e)),
        };
        let mut fresh_req = match Request::new(Method::Get, url) {
            Ok(req) => req,
            Err(e)  => return Err(e)
        };
        let mut auth_s = String::new();
        auth_s.push_str("Bearer ");
        auth_s.push_str(self.auth);
        fresh_req.headers_mut().set(ContentType("application/json".parse().unwrap()));
        fresh_req.headers_mut().set(Authorization(auth_s));
        let streaming_req = try!(fresh_req.start());
        let mut response = try!(streaming_req.send());
        let mut s = String::new();
        try!(response.read_to_string(&mut s));
        Ok(s)
    }

    pub fn retrieve(&self) -> Result<response::Account, String> {
        match self.retrieve_json() {
            Ok(ref s) => {
                match json::from_str::<Value>(s) {
                    Ok(obj) => {
                        match obj.find("account") {
                            Some(a) => {
                                match json::from_value(a.clone()) {
                                    Ok(a) => Ok(a),
                                    Err(e) => Err(e.into())
                                }
                            },
                            None => Err(json::Error::MissingFieldError("account").into())
                        }
                    },
                    Err(e) => Err(e.into())
                }
            },
            Err(e) => Err(e.into())
        }
    }

    pub fn action_request<'a>(&self, id: &'a str) -> Action {

    }

    pub fn action_request<'a>(&self, id: &'a str) -> hyper::Result<Request<Fresh>> {
        let url = match Url::parse(self.url) {
            Ok(url) => url,
            Err(e) => return Err(Error::Uri(e)),
        };
        let mut fresh_req = match Request::new(Method::Get, url) {
            Ok(req) => req,
            Err(e)  => return Err(e)
        };
        let mut auth_s = String::new();
        auth_s.push_str("Bearer ");
        auth_s.push_str(self.auth);
        fresh_req.headers_mut().set(ContentType("application/json".parse().unwrap()));
        fresh_req.headers_mut().set(Authorization(auth_s));
        Ok(fresh_req)
    }

    pub fn retrieve_action_json<'a>(&self, id: &'a str) -> hyper::Result<String> {
        let url = match Url::parse(self.url) {
            Ok(url) => url,
            Err(e) => return Err(Error::Uri(e)),
        };
        let mut fresh_req = match Request::new(Method::Get, url) {
            Ok(req) => req,
            Err(e)  => return Err(e)
        };
        let mut auth_s = String::new();
        auth_s.push_str("Bearer ");
        auth_s.push_str(self.auth);
        fresh_req.headers_mut().set(ContentType("application/json".parse().unwrap()));
        fresh_req.headers_mut().set(Authorization(auth_s));
        let streaming_req = try!(fresh_req.start());
        let mut response = try!(streaming_req.send());
        let mut s = String::new();
        try!(response.read_to_string(&mut s));
        Ok(s)
    }

    pub fn retrieve_action<'a>(&self, id: &'a str) -> Result<response::Action, String> {
        match self.retrieve_json() {
            Ok(ref s) => {
                match json::from_str::<Value>(s) {
                    Ok(obj) => {
                        match obj.find("account") {
                            Some(a) => {
                                match json::from_value(a.clone()) {
                                    Ok(a) => Ok(a),
                                    Err(e) => Err(e.into())
                                }
                            },
                            None => Err(json::Error::MissingFieldError("action").into())
                        }
                    },
                    Err(e) => Err(e.into())
                }
            },
            Err(e) => Err(e.into())
        }
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
