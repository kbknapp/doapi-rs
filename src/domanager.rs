use std::marker::PhantomData;

use response;
use request::RequestBuilder;

pub struct DoManager<'t> {
    auth: &'t str,
}

impl<'t> DoManager<'t> {
    pub fn with_token(t: &'t str) -> DoManager<'t> {
        DoManager {
            auth: t
        }
    }

    pub fn account(&self) -> RequestBuilder<'t, response::Account> {
        RequestBuilder {
            auth: self.auth,
            url: "https://api.digitalocean.com/v2/account".to_owned(),
            resp_t: PhantomData
        }
    }

    pub fn regions(&self) -> RequestBuilder<'t, response::Regions> {
        RequestBuilder {
            auth: self.auth,
            url: "https://api.digitalocean.com/v2/regions".to_owned(),
            resp_t: PhantomData
        }
    }

    pub fn sizes(&self) -> RequestBuilder<'t, response::Sizes> {
        RequestBuilder {
            auth: self.auth,
            url: "https://api.digitalocean.com/v2/sizes".to_owned(),
            resp_t: PhantomData
        }
    }

    pub fn image(&self, id: &str) -> RequestBuilder<'t, response::Image> {
        RequestBuilder {
            auth: self.auth,
            url: format!("https://api.digitalocean.com/v2/images/{}", id),
            resp_t: PhantomData
        }
    }

    pub fn images(&self) -> RequestBuilder<'t, response::Images> {
        RequestBuilder {
            auth: self.auth,
            url: "https://api.digitalocean.com/v2/images".to_owned(),
            resp_t: PhantomData
        }
    }

    pub fn ssh_keys(&self) -> RequestBuilder<'t, response::SshKeys> {
        RequestBuilder::with_auth(self.auth)
    }

    pub fn dns(&self) -> RequestBuilder<'t, response::DnsRecord> {
        RequestBuilder::with_auth(self.auth)
    }

    pub fn droplet(&self, id: &str) -> RequestBuilder<'t, response::Droplet> {
        RequestBuilder {
            auth: self.auth,
            url: format!("https://api.digitalocean.com/v2/droplet/{}", id),
            resp_t: PhantomData
        }
    }

    pub fn droplets(&self) -> RequestBuilder<'t, response::Droplets> {
        RequestBuilder {
            auth: self.auth,
            url: "https://api.digitalocean.com/v2/droplet".to_owned(),
            resp_t: PhantomData
        }
    }

    pub fn domains(&self) -> RequestBuilder<'t, response::Domains> {
        RequestBuilder {
            auth: self.auth,
            url: "https://api.digitalocean.com/v2/domains".to_owned(),
            resp_t: PhantomData
        }
    }

    pub fn domain(&self, id: &str) -> RequestBuilder<'t, response::Domain> {
        RequestBuilder {
            auth: self.auth,
            url: format!("https://api.digitalocean.com/v2/domains/{}", id),
            resp_t: PhantomData
        }
    }
}
