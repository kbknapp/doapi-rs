use std::marker::PhantomData;
use std::iter::Iterator;
use std::fmt;

use serde::{json, Deserialize};

use response::{self, RawPagedResponse, NamedResponse};
use request::{BaseRequest, DoRequest};
use request::PagedRequest;

pub struct RequestBuilder<'t, T> {
    pub auth: &'t str,
    pub url: String,
    pub resp_t: PhantomData<*const T>
}

impl<'t, T> RequestBuilder<'t, T> {
    pub fn with_auth(t: &'t str) -> RequestBuilder<'t, T> {
        RequestBuilder {
            auth: t,
            url: String::new(),
            resp_t: PhantomData
        }
    }
}

impl<'t, T> fmt::Display for RequestBuilder<'t, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "method: GET\n\
                content-type: application/json\n\
                authorization: Bearer {}\n\
                url: {}", self.auth, if !self.url.is_empty() { self.url.clone() } else { "None".to_owned() })
    }
}

impl<'t, T> BaseRequest for RequestBuilder<'t, T> {
    fn auth(&self) -> &str {
        self.auth
    }
    fn url(&self) -> &str {
        &self.url[..]
    }
}

// Can't use because of impl for DoRequest<Vec<T>>, waiting on negative trait bounds
// impl<'t, T: !Iterator> DoRequest<T> for RequestBuilder<'t, T> { }

impl<'t, I> PagedRequest for RequestBuilder<'t, Vec<I>> 
                                              where I: Deserialize + NamedResponse {
    type Item = I;
    fn retrieve_single_page(&self, url: String) -> Result<RawPagedResponse<I>, String> {
        if url.is_empty() { return Err("No URL provided".to_owned()) }
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
                        println!("DEBUG: error {}", e.to_string());
                        println!("DEBUG: Raw JSON {}", json_str);
                        return Err(e.to_string())
                    }
                }
            },
            Err(e) => return Err(e.to_string())
        }
    }
}

impl<'t, I> DoRequest<Vec<I>> for RequestBuilder<'t, Vec<I>>
                                where I: Deserialize + NamedResponse {
    fn retrieve(&self) -> Result<Vec<I>, String> {
        match self.retrieve_json() {
            Ok(ref s) => {
                 // FIXME \/ \/
                let mut name = <I as NamedResponse>::name().into_owned();
                name.push('s');
                let re = regex!(&format!("\"{}\"", name));
                let res = &re.replace(&s[..], "\"collection\"");
                match json::from_str::<response::RawPagedResponse<I>>(res) {
                // FIXME ^^
                    Ok(mut val) => {
                        let mut regs = vec![];
                        regs.append(&mut val.collection);
                        let mut url = if val.links.pages.is_some() && val.links.pages.clone().unwrap().next.is_some() {
                            val.links.pages.clone().unwrap().next.clone().unwrap()
                        } else {
                            String::new()
                        };
                        println!("(BEFORE) URL: {}", &url[..]);
                        while let Ok(mut page) = self.retrieve_single_page(url.clone()) {
                            println!("(IN) URL: {}", &url[..]);
                            regs.append(&mut page.collection);
                            url = if page.links.pages.is_some() && page.links.pages.clone().unwrap().next.is_some() {
                                page.links.pages.clone().unwrap().next.clone().unwrap()
                            } else {
                                String::new()
                            };
                        }
                        println!("(AFTER) URL: {}", &url[..]);
                        Ok(regs)
                    },
                    Err(e) => Err(e.to_string())
                }
            },
            Err(e) => Err(e.to_string())
        }
    }
}