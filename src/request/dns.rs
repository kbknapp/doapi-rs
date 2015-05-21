use std::fmt;

use response;
use request::{Request, DoRequest};

pub struct Dns<'t>(DoRequest<'t>);

impl<'t> Dns<'t> {
    pub fn with_token(token: &'t str) -> Dns {
        Dns( DoRequest::new("https://api.digitalocean.com/v2/domains".to_owned(), token) )
    }

    pub fn create(&self, rec: &DnsRecord) -> Dns {
        unimplemented!()
    }

    pub fn records(&self) -> Dns {
        unimplemented!()
    }

    pub fn update(&self, id: &str) -> Dns {
        unimplemented!()
    }

    pub fn update(&self, id: &str) -> Dns {
        unimplemented!()
    }

    pub fn delete(&self, id: &str) -> Dns {
        unimplemented!()
    }
}

impl<'t> Request for Dns<'t> {
    type RespT = response::Dns;
    fn auth(&self) -> &str {
        self.0.auth_token
    }
    fn url(&self) -> &str {
        &self.0.url_str[..]
    }
    fn retrieve(&self) -> Result<response::Dns, String> {
        self.0.retrieve_obj::<response::Dns>("dns".to_owned())
    }
}

impl<'t> fmt::Display for Dns<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub struct DnsRecord;