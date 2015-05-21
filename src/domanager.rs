use request::{Account, Dns, Regions, Sizes, Images, SshKeys, Droplets, Domains};

pub struct DoManager<'t> {
    auth_token: &'t str
}

impl<'t> DoManager<'t> {
    pub fn with_token(t: &'t str) -> DoManager {
        DoManager {
            auth_token: t
        }
    }

    pub fn account(&self) -> Account {
        Account::with_token(self.auth_token)
    }

    pub fn regions(&self) -> Regions {
        Regions::with_token(self.auth_token)
    }

    pub fn sizes(&self) -> Sizes {
        Sizes::with_token(self.auth_token)
    }

    pub fn images(&self) -> Images {
        Images::with_token(self.auth_token)
    }

    pub fn ssh_keys(&self) -> SshKeys {
        SshKeys::with_token(self.auth_token)
    }

    pub fn droplets(&self) -> Droplets {
        Droplets::with_token(self.auth_token)
    }

    pub fn domains(&self) -> Domains {
        Domains::with_token(self.auth_token)
    }

    pub fn dns(&self) -> Dns {
        Dns::with_token(self.auth_token)
    }
}
