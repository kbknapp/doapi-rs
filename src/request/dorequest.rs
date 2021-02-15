use std::convert::TryFrom;

use reqwest::Method;
use serde::de::DeserializeOwned;
use serde_json::{self, Value};

use url::Url;

use crate::response::{self, DoError, NamedResponse};

pub trait BaseRequest {
    fn url(&self) -> &Url;
    fn auth(&self) -> &str;
    fn client(&self) -> &reqwest::blocking::Client;
    fn method(&self) -> Method;
    fn body(&self) -> Option<String>;
}

pub trait DoRequest<T>: BaseRequest
where
    T: DeserializeOwned + NamedResponse,
{
    fn request(&self) -> reqwest::blocking::RequestBuilder {
        self.client()
            .request(self.method(), self.url().clone())
            .bearer_auth(self.auth())
    }

    fn retrieve_json(&self) -> reqwest::Result<String> {
        self.retrieve_raw_response()?.text()
    }

    fn retrieve_raw_response(&self) -> reqwest::Result<reqwest::blocking::Response> {
        let body = if let Some(body) = self.body() {
            body
        } else {
            String::new()
        };
        self.request()
            .body(body)
            .header("Content-Type", "application/json")
            .send()
    }

    fn retrieve_header(&self) -> Result<response::HeaderOnly, String> {
        match self.retrieve_raw_response() {
            Ok(response) => response::HeaderOnly::try_from(response),
            Err(error) => Err(error.to_string()),
        }
    }

    fn retrieve_obj(&self, obj: String) -> Result<T, String> {
        match self.retrieve_json() {
            Ok(s) => match serde_json::from_str::<Value>(&s) {
                Ok(value) => match value.get(&obj) {
                    Some(value) => match serde_json::from_value(value.clone()) {
                        Ok(value) => Ok(value),
                        Err(error) => Err(error.to_string()),
                    },
                    None => match serde_json::from_value::<DoError>(value.clone()) {
                        Ok(error) => Err(error.to_string()),
                        Err(error) => Err(error.to_string()), // TODO - FIX THIS.
                    },
                },
                Err(error) => Err(error.to_string()),
            },

            Err(error) => Err(error.to_string()),
        }
    }

    fn retrieve(&self) -> Result<T, String> {
        self.retrieve_obj(<T as response::NamedResponse>::name().into_owned())
    }
}
