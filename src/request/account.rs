use std::fmt;

use response;
use request::Request;
use request::basic::BasicRequest;
use request::action::{Action, Actions};

pub struct Account<'t>(BasicRequest<'t>);

impl<'t> Account<'t> {
    pub fn with_token(token: &'t str) -> Account {
        Account( BasicRequest::new("https://api.digitalocean.com/v2/account".to_owned(), token) )
    }

    pub fn action(&self, id: &str) -> Action {
        Action::new(format!("https://api.digitalocean.com/v2/actions/{}", id), self.0.auth_token)
    }

    pub fn actions(&self) -> Actions {
        Actions::new("https://api.digitalocean.com/v2/actions".to_owned(), self.0.auth_token)
    }

}

impl<'t> Request for Account<'t> {
    type RespT = response::Account;
    fn auth(&self) -> &str {
        self.0.auth_token
    }
    fn url(&self) -> &str {
        &self.0.url_str[..]
    }
    fn retrieve(&self) -> Result<response::Account, String> {
        self.0.retrieve_obj::<response::Account>("account".to_owned())
    }
}

impl<'t> fmt::Display for Account<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
