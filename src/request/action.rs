
use std::fmt;

use serde::json;

use request::basic::BasicRequest;
use request::Request;
use response::{self, RawActions};

pub struct Action<'t>(BasicRequest<'t>);
pub struct Actions<'t>(BasicRequest<'t>);

impl<'t> Action<'t> {
    pub fn new(url: String, token: &'t str) -> Action<'t>{
        Action(BasicRequest::new(url, token))
    }
}

impl<'t> Request for Action<'t> {
    type RespT = response::Action;
    fn auth(&self) -> &str {
        self.0.auth_token
    }
    fn url(&self) -> &str {
        &self.0.url_str[..]
    }
    fn retrieve(&self) -> Result<response::Action, String> {
        self.0.retrieve_obj("action".to_owned())
    }
}

impl<'t> fmt::Display for Action<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'t> Actions<'t> {
    pub fn new(url: String, token: &'t str) -> Actions<'t> {
        Actions(BasicRequest::new(url, token))
    }

    fn retrieve_single_page(url: String, token: &str) -> Result<RawActions, String> {
        let a = Actions(BasicRequest::new(url, token));

        match a.retrieve_json() {
            Ok(ref s) => {
                match json::from_str::<RawActions>(s) {
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

impl<'t> Request for Actions<'t> {
    type RespT = response::Actions;
    fn auth(&self) -> &str {
        &self.0.auth_token[..]
    }
    fn url(&self) -> &str {
        &self.0.url_str[..]
    }
    fn retrieve(&self) -> Result<response::Actions, String> {
        match self.retrieve_json() {
            Ok(ref s) => {
                match json::from_str::<RawActions>(s) {
                    Ok(ref mut val) => {
                        let mut acts = response::Actions {
                            actions: vec![]
                        };
                        acts.actions.append(&mut val.actions);
                        while let Ok(ref mut val) = Actions::retrieve_single_page(val.links.pages.next.clone(), self.0.auth_token) {
                            acts.actions.append(&mut val.actions);
                        }
                        Ok(acts)
                    },
                    Err(e) => Err(e.to_string())
                }
            },
            Err(e) => Err(e.to_string())
        }
    }
}

impl<'t> fmt::Display for Actions<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }

}
