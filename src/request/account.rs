use std::fmt;

use response;
use request::{DoRequest, NamedRequest, Request};
use request::action::{Action, Actions};

pub struct Account<'t>(DoRequest<'t>);

impl<'t> Account<'t> {
    pub fn with_token(token: &'t str) -> Account {
        Account( DoRequest::new("https://api.digitalocean.com/v2/account".to_owned(), token) )
    }

    pub fn action(&self, id: &str) -> Action {
        Action::new(format!("https://api.digitalocean.com/v2/actions/{}", id), self.0.auth_token)
    }

    pub fn actions(&self) -> Actions {
        Actions::new("https://api.digitalocean.com/v2/actions".to_owned(), self.0.auth_token)
    }

}

impl<'t> NamedRequest for Account<'t> {
    type RespT = response::Account;
    fn name(&self) -> &str {
        "account"
    }
}

impl<'t> fmt::Display for Account<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
