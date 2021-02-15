//use std::fmt;
use std::marker::PhantomData;

use reqwest::Method;
use serde::de::DeserializeOwned;
use url::Url;

use crate::response::{self, NamedResponse, NotArray, RawPagedResponse};
use crate::DoManager;

use crate::request::PagedRequest;
use crate::request::{BaseRequest, DoRequest};

#[derive(Debug)]
pub struct RequestBuilder<'a, 't, T> {
    pub domgr: &'t DoManager<'a>,
    pub method: Method,
    pub url: url::Url,
    pub resp_t: PhantomData<*const T>,
    pub body: Option<String>,
}

impl<'a, 't, T> RequestBuilder<'a, 't, T> {
    pub fn body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }
}

impl<'a, 't, T> RequestBuilder<'a, 't, T> {
    pub fn new(domgr: &'t DoManager<'a>, path: &str) -> RequestBuilder<'a, 't, T> {
        RequestBuilder {
            domgr,
            method: Method::GET,
            url: domgr.endpoint_url.join(path).unwrap(),
            resp_t: PhantomData,
            body: None,
        }
    }
}

// impl<'t, T> fmt::Display for RequestBuilder<'t, T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f,
//                "method: {}\n\
//                 content-type: application/json\n\
//                 authorization: Bearer {}\n\
//                 url: {}\n\
//                 body: {}\n",
//                self.method,
//                self.auth,
//                if !self.url.is_empty() {
//                    &self.url
//                } else {
//                    "None"
//                },
//                if let Some(ref bdy) = self.body {
//                    bdy
//                } else {
//                    "None"
//                })
//     }
// }

impl<'a, 't, T> BaseRequest for RequestBuilder<'a, 't, T> {
    fn auth(&self) -> &str {
        &self.domgr.auth
    }
    fn client(&self) -> &reqwest::blocking::Client {
        &self.domgr.client
    }

    fn url(&self) -> &Url {
        &self.url
    }
    fn method(&self) -> Method {
        self.method.clone()
    }
    fn body(&self) -> Option<String> {
        self.body.clone()
    }
}

// Can't use because of impl for DoRequest<Vec<T>>, waiting on negative trait
// bounds
// impl<'t, T: !Iterator> DoRequest<T> for RequestBuilder<'t, T> { }

impl<'a, 't, I> PagedRequest for RequestBuilder<'a, 't, Vec<I>>
where
    I: DeserializeOwned + NamedResponse + NotArray,
{
    type Item = I;
    fn retrieve_single_page(&self, url: Url) -> Result<RawPagedResponse<I>, String> {
        let request_builder: RequestBuilder<'_, '_, Vec<I>> =
            self.domgr.request_builder(Method::GET, url);

        match request_builder.retrieve_json() {
            Ok(ref s) => {
                let mut name = <I as NamedResponse>::name().into_owned();
                name.push('s');
                let re = regex::Regex::new(&format!("\"{}\"", name)).unwrap();
                let json_str = re.replace(&s[..], "\"collection\"");
                match serde_json::from_str::<response::RawPagedResponse<I>>(&json_str) {
                    Ok(value) => Ok(value),
                    Err(error) => Err(error.to_string()),
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

impl<'a, 't, I> DoRequest<Vec<I>> for RequestBuilder<'a, 't, Vec<I>>
where
    I: DeserializeOwned + NamedResponse + NotArray,
{
    fn retrieve(&self) -> Result<Vec<I>, String> {
        match self.retrieve_json() {
            Ok(ref s) => {
                let mut name = <I as NamedResponse>::name().into_owned();
                name.push('s');
                let re = regex::Regex::new(&format!("\"{}\"", name)).unwrap();
                let res = re.replace(&s[..], "\"collection\"");
                match serde_json::from_str::<response::RawPagedResponse<I>>(&res) {
                    Ok(mut val) => {
                        let mut regs = Vec::new();
                        regs.append(&mut val.collection);
                        let mut url = if val.links.pages.is_some()
                            && val.links.pages.clone().unwrap().next.is_some()
                        {
                            Url::parse(&val.links.pages.unwrap().next.unwrap()).unwrap()
                        } else {
                            self.domgr.endpoint_url.clone()
                        };

                        while let Ok(mut page) = self.retrieve_single_page(url) {
                            regs.append(&mut page.collection);
                            url = if page.links.pages.is_some()
                                && page.links.pages.clone().unwrap().next.is_some()
                            {
                                Url::parse(&page.links.pages.clone().unwrap().next.clone().unwrap())
                                    .unwrap()
                            } else {
                                self.domgr.endpoint_url.clone()
                            };
                        }
                        Ok(regs)
                    }
                    Err(_) => self.retrieve_obj(name),
                }
            }
            Err(error) => Err(error.to_string()),
        }
    }
}

impl<'a, 't> DoRequest<response::HeaderOnly> for RequestBuilder<'a, 't, response::HeaderOnly> {
    fn retrieve(&self) -> Result<response::HeaderOnly, String> {
        self.retrieve_header()
    }
}
