use std::fmt;

use request::Request;

pub struct BasicRequest<'t> {
    pub url_str: String,
    pub auth_token: &'t str
}

impl<'t> BasicRequest<'t> {
    pub fn new(url: String, token: &'t str) -> BasicRequest<'t> {
        BasicRequest {
            url_str: url,
            auth_token: token
        }
    }
}

impl<'t> Request for BasicRequest<'t> {
    type RespT = ();
    fn auth(&self) -> &str {
        self.auth_token
    }
    fn url(&self) -> &str {
        &self.url_str[..]
    }

    fn retrieve(&self) -> Result<(), String> {
        Err("Cannot call .retrieve() on type BasicRequest".to_owned())
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
