use std::marker::PhantomData;
use std::fmt;

use serde::{json, Deserialize};

use response::{self, RawPagedResponse, Pages, NamedResponse};
use request::{BaseRequest, DoRequest};
use request::PagedRequest;

pub struct RequestBuilder<'t, T> {
    pub auth: &'t str,
    pub url: Option<String>,
    pub resp_t: PhantomData<*const T>
}

impl<'t, T> RequestBuilder<'t, T> {
    pub fn with_auth(t: &'t str) -> RequestBuilder<'t, T> {
        RequestBuilder {
            auth: t,
            url: None,
            resp_t: PhantomData
        }
    }
}

impl<'t, T> fmt::Display for RequestBuilder<'t, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "method: GET\n\
                content-type: application/json\n\
                authorization: Bearer {}\n\
                url: {}", self.auth, if self.url.is_some() { self.url.clone().unwrap() } else { "None".to_owned() })
    }
}

impl<'t, T> BaseRequest for RequestBuilder<'t, T> {
    fn auth(&self) -> &str {
        self.auth
    }
    fn url(&self) -> String {
        if let Some(url) = self.url.clone() {
            url
        } else {
            "".to_owned()
        }
    }
}


impl<'t, I> PagedRequest for RequestBuilder<'t, Vec<I>> 
                                              // where T: Deserialize + NewIter<Item=I> + NamedResponse,
                                                    where I: Deserialize + NamedResponse {
    type Item = I;
    fn retrieve_single_page(&self, url: String) -> Result<RawPagedResponse<I>, String> {
        let mut rb: RequestBuilder<'t, Vec<I>> = RequestBuilder::with_auth(self.auth);
        rb.url = Some(url);
        match rb.retrieve_json() {
            Ok(ref s) => {
                 // FIXME \/ \/
                let mut name = <I as NamedResponse>::name().into_owned();
                name.push('s');
                let re = regex!(&format!("\"{}\"", name));
                match json::from_str::<response::RawPagedResponse<I>>(&re.replace(&s[..], "\"collection\"")) {
                // FIXME ^^
                    Ok(val) => {
                        Ok(val)
                    },
                    Err(e) => Err(e.to_string())
                }
            },
            Err(e) => Err(e.to_string())
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
                        while let Ok(mut val) = self.retrieve_single_page(val.links.pages.clone().unwrap_or(Pages{
                            next: String::new(),
                            last: String::new()
                        }).next.clone()) {
                            regs.append(&mut val.collection);
                        }
                        Ok(regs)
                    },
                    Err(e) => Err(e.to_string())
                }
            },
            Err(e) => Err(e.to_string())
        }
    }
}