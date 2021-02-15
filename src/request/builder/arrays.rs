use std::borrow::Cow;

use crate::request::{DoRequest, RequestBuilder};
use crate::response;

impl response::NamedResponse for String {
    fn name<'a>() -> Cow<'a, str> {
        "".into()
    }
}

impl<'a, 't> DoRequest<response::ResponseStringArray>
    for RequestBuilder<'a, 't, response::ResponseStringArray>
{
    fn retrieve_obj(&self, _: String) -> Result<response::ResponseStringArray, String> {
        match self.retrieve_json() {
            Ok(s) => match serde_json::from_str::<response::ResponseStringArray>(&s) {
                Ok(ob) => Ok(ob),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}

impl<'a, 't> DoRequest<response::Neighbors> for RequestBuilder<'a, 't, response::Neighbors> {}
