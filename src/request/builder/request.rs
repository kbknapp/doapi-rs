use std::marker::PhantomData;
use std::iter::Iterator;
use std::collections::HashMap;
use std::fmt;

use serde::{json, Deserialize};
use hyper::method::Method;

use response::{self, RawPagedResponse, NamedResponse};
use request::{BaseRequest, DoRequest};
use request::PagedRequest;

pub struct RequestBuilder<'t, T> {
    pub auth: &'t str,
    pub method: Method,
    pub url: String,
    pub resp_t: PhantomData<*const T>,
    pub body: Option<RequestBody>
}

pub type RequestBody = HashMap<&'static str, String>;
pub trait ToJsonString {
    fn to_json(&self) -> String;
}

impl ToJsonString for RequestBody {
    fn to_json(&self) -> String {
        let mut s = String::with_capacity(100);
        s.push('{');
        for (k, v) in self.iter() {
            s.push('"');
            s.push_str(&k[..]);
            s.push('"');
            s.push(':');
            s.push('"');
            s.push_str(&v[..]);
            s.push('"');
            s.push(',');
        }
        s.push('}');
        s.shrink_to_fit();
        s
    }
}

impl<'t, T> RequestBuilder<'t, T> {
    pub fn with_auth(auth: &'t str) -> RequestBuilder<'t, T> {
        RequestBuilder {
            auth: auth,
            method: Method::Get,
            url: String::new(),
            resp_t: PhantomData,
            body: None
        }
    }
    pub fn new<S>(auth: &'t str, url: S)
                  -> RequestBuilder<'t, T>
                  where S: Into<String> {
        RequestBuilder {
            auth: auth,
            method: Method::Get,
            url: url.into(),
            resp_t: PhantomData,
            body: None
        }
    }
}

impl<'t, T> fmt::Display for RequestBuilder<'t, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "method: {}\n\
                content-type: application/json\n\
                authorization: Bearer {}\n\
                url: {}", self.method, self.auth, if !self.url.is_empty() { self.url.clone() } else { "None".to_owned() })
    }
}

impl<'t, T> BaseRequest for RequestBuilder<'t, T> {
    fn auth(&self) -> &str {
        self.auth
    }
    fn url(&self) -> &str {
        &self.url[..]
    }
    fn method(&self) -> Method {
        self.method.clone()
    }
    fn body(&self) -> Option<String> {
        if let Some(ref hm) = self.body {
            Some(hm.to_json())
        } else {
            None
        }
    }
}

// Can't use because of impl for DoRequest<Vec<T>>, waiting on negative trait bounds
// impl<'t, T: !Iterator> DoRequest<T> for RequestBuilder<'t, T> { }

impl<'t, I> PagedRequest for RequestBuilder<'t, Vec<I>>
                                              where I: Deserialize + NamedResponse {
    type Item = I;
    fn retrieve_single_page(&self, url: String) -> Result<RawPagedResponse<I>, String> {
        debug!("Inside retrieve_single_page() with url: {}", &url[..]);
        if url.is_empty() {
            debug!("No url provided");
            return Err("No URL provided".to_owned())
        }
        let mut rb: RequestBuilder<'t, Vec<I>> = RequestBuilder::with_auth(self.auth);
        rb.url = url.clone();
        match rb.retrieve_json() {
            Ok(ref s) => {
                 // FIXME \/ \/
                let mut name = <I as NamedResponse>::name().into_owned();
                name.push('s');
                let re = regex!(&format!("\"{}\"", name));
                let json_str = &re.replace(&s[..], "\"collection\"");
                match json::from_str::<response::RawPagedResponse<I>>(json_str) {
                // FIXME ^^
                    Ok(val) => {
                        return Ok(val)
                    },
                    Err(e) => {
                        debug!("Error getting objects: {}", e.to_string());
                        return Err(e.to_string())
                    }
                }
            },
            Err(e) => {
                debug!("Error getting json: {}", e.to_string());
                return Err(e.to_string())
            }
        }
    }
}

impl<'t, I> DoRequest<Vec<I>> for RequestBuilder<'t, Vec<I>>
                                where I: Deserialize + NamedResponse {
    fn retrieve(&self) -> Result<Vec<I>, String> {
        debug!("Inside retrieve() for  paged request");
        debug!("Getting json...");
        match self.retrieve_json() {
            Ok(ref s) => {
                debug!("Sucessfully retrieved json");
                 // FIXME \/ \/
                let mut name = <I as NamedResponse>::name().into_owned();
                name.push('s');
                debug!("Replacing {} with collection", &name[..]);
                let re = regex!(&format!("\"{}\"", name));
                let res = &re.replace(&s[..], "\"collection\"");
                debug!("Getting object from json string");
                match json::from_str::<response::RawPagedResponse<I>>(res) {
                // FIXME ^^
                    Ok(mut val) => {
                        debug!("Sucessfully retrieved object");
                        let mut regs = vec![];
                        regs.append(&mut val.collection);
                        let mut url = if val.links.pages.is_some() && val.links.pages.clone().unwrap().next.is_some() {
                            val.links.pages.clone().unwrap().next.clone().unwrap()
                        } else {
                            String::new()
                        };
                        debug!("Next page URL: {}", &url[..]);
                        while let Ok(mut page) = self.retrieve_single_page(url.clone()) {
                            regs.append(&mut page.collection);
                            url = if page.links.pages.is_some() && page.links.pages.clone().unwrap().next.is_some() {
                                page.links.pages.clone().unwrap().next.clone().unwrap()
                            } else {
                                String::new()
                            };
                            debug!("Next page URL: {}", &url[..]);
                        }
                        Ok(regs)
                    },
                    Err(e) => {
                        debug!("Error getting object: {}", e.to_string());
                        Err(e.to_string())
                    }
                }
            },
            Err(e) => {
                debug!("Error getting json: {}", e.to_string());
                Err(e.to_string())
            }
        }
    }
}

impl<'t> DoRequest<response::HeaderOnly> for RequestBuilder<'t, response::HeaderOnly> {
    fn retrieve(&self) -> Result<response::HeaderOnly, String> {
        self.retrieve_header()
    }
}