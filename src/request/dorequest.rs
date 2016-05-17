use std::io::{Read, Write};

use hyper::client;
use hyper::{self, Error, Url};
use hyper::method::Method;
use hyper::net::Fresh;
use hyper::header::{Authorization, ContentType};

use serde_json::{self, Value};
use serde::de::Deserialize;

use response::{self, DoError, NamedResponse};

pub trait BaseRequest {
    fn url(&self) -> &str;
    fn auth(&self) -> &str;
    fn method(&self) -> Method;
    fn body(&self) -> Option<String>;
}

pub trait DoRequest<T> : BaseRequest
              where T: Deserialize + NamedResponse {
    fn request(&self) -> hyper::Result<client::Request<Fresh>> {
        let url = match Url::parse(self.url()) {
            Ok(url) => url,
            Err(e) => return Err(Error::Uri(e)),
        };
        let mut fresh_req = match client::Request::new(self.method(), url) {
            Ok(req) => req,
            Err(e) => return Err(e),
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
        let mut fresh_req = match client::Request::new(self.method(), url) {
            Ok(req) => req,
            Err(e) => return Err(e),
        };
        let mut auth_s = String::new();
        auth_s.push_str("Bearer ");
        auth_s.push_str(self.auth());
        fresh_req.headers_mut().set(ContentType("application/json".parse().unwrap()));
        fresh_req.headers_mut().set(Authorization(auth_s));
        let mut streaming_req = try!(fresh_req.start());
        if let Some(ref b) = self.body() {
            streaming_req.write(b.as_bytes()).unwrap();
        }
        let mut response = try!(streaming_req.send());
        let mut s = String::new();
        try!(response.read_to_string(&mut s));
        Ok(s)
    }

    fn retrieve_raw_response(&self) -> hyper::Result<client::response::Response> {
        let url = match Url::parse(self.url()) {
            Ok(url) => url,
            Err(e) => return Err(Error::Uri(e)),
        };
        let mut fresh_req = match client::Request::new(self.method(), url) {
            Ok(req) => req,
            Err(e) => return Err(e),
        };
        let mut auth_s = String::new();
        auth_s.push_str("Bearer ");
        auth_s.push_str(self.auth());
        fresh_req.headers_mut().set(ContentType("application/json".parse().unwrap()));
        fresh_req.headers_mut().set(Authorization(auth_s));
        let mut streaming_req = try!(fresh_req.start());
        if let Some(ref b) = self.body() {
            streaming_req.write(b.as_bytes()).unwrap();
        }
        let response = try!(streaming_req.send());
        Ok(response)
    }

    fn retrieve_header(&self) -> Result<response::HeaderOnly, String> {
        debug!("Inside retrieve_header()");
        debug!("Getting raw response...");
        match self.retrieve_raw_response() {
            Ok(resp) => {
                let header = try!(response::HeaderOnly::from_response(resp));
                Ok(header)
            }
            Err(e) => {
                debug!("Error getting json: {}", e.to_string());
                Err(e.to_string())
            }
        }
    }

    fn retrieve_obj(&self, obj: String) -> Result<T, String> {
        debug!("inside retrieve_obj() for regular type");
        match self.retrieve_json() {
            Ok(ref s) => {
                match serde_json::from_str::<Value>(s) {
                    Ok(ob) => {
                        match ob.find(&obj) {
                            Some(t) => {
                                match serde_json::from_value(t.clone()) {
                                    Ok(t) => Ok(t),
                                    Err(e) => Err(e.to_string()),
                                }
                            }
                            None => {
                                match serde_json::from_value::<DoError>(ob.clone()) {
                                    Ok(err) => Err(err.to_string()),
                                    Err(e) => Err(e.to_string()),
                                }
                            }
                        }
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }

    fn retrieve(&self) -> Result<T, String> {
        debug!("Inside retrieve() for regular type");
        self.retrieve_obj(<T as response::NamedResponse>::name().into_owned())
    }
}
