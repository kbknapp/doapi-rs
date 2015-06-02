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
        RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/account")
    }

    pub fn regions(&self) -> RequestBuilder<'t, response::Regions> {
        RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/regions")
    }

    pub fn sizes(&self) -> RequestBuilder<'t, response::Sizes> {
        RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/sizes")
    }

    pub fn image(&self, id: &str) -> RequestBuilder<'t, response::Image> {
        RequestBuilder::new(self.auth, format!("https://api.digitalocean.com/v2/images/{}", id))
    }

    pub fn images(&self) -> RequestBuilder<'t, response::Images> {
        RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/images")
    }

    pub fn ssh_keys(&self) -> RequestBuilder<'t, response::SshKeys> {
        RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/account/keys")
    }

    pub fn ssh_key(&self, id: &str) -> RequestBuilder<'t, response::SshKey> {
        RequestBuilder::new(self.auth, format!("https://api.digitalocean.com/v2/account/keys/{}", id))
    }


    pub fn droplet(&self, id: &str) -> RequestBuilder<'t, response::Droplet> {
        RequestBuilder::new(self.auth, format!("https://api.digitalocean.com/v2/droplet/{}", id))
    }

    pub fn droplets(&self) -> RequestBuilder<'t, response::Droplets> {
        RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/droplet")
    }

    pub fn domains(&self) -> RequestBuilder<'t, response::Domains> {
        RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/domains")
    }

    pub fn domain(&self, id: &str) -> RequestBuilder<'t, response::Domain> {
        RequestBuilder::new(self.auth, format!("https://api.digitalocean.com/v2/domains/{}", id))
    }
}
