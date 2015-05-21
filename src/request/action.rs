
use std::fmt;

use serde::json;

use request::{NamedRequest, Request, DoRequest};
use response::{self, RawActions};

pub struct Action<'t>(DoRequest<'t>);
pub struct Actions<'t>(DoRequest<'t>);

impl<'t> Action<'t> {
    pub fn new(url: String, token: &'t str) -> Action<'t>{
        Action(DoRequest::new(url, token))
    }
}

impl<'t> NamedRequest for Action<'t> {
    type RespT = response::Action;
    fn name(&self) -> &str {
        "action"
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

impl<'t> NamedRequest for Actions<'t> {
    type RespT = response::Actions;
    fn name(&self) -> &str {
        "actions"
    }
}

impl<'t> Request for Actions<'t> {
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
