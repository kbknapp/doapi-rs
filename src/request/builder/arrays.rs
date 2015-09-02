use std::borrow::Cow;

use serde_json;

use request::DoRequest;
use request::RequestBuilder;
use response;

impl response::NamedResponse for String {
    fn name<'a>() -> Cow<'a, str> {
        "".into()
    }
}

impl<'t> DoRequest<response::ResponseStringArray> for RequestBuilder<'t, response::ResponseStringArray> {
    #[allow(unused_variables)]
    fn retrieve_obj(&self, obj: String) -> Result<response::ResponseStringArray, String> {
        debug!("Inside retrieve_obj() of ResponseStringArray");
        debug!("Retrieveing JSON");
        match self.retrieve_json() {
            Ok(ref s) => {
                debug!("Success");
                debug!("Retrieving Value");
                match serde_json::from_str::<response::ResponseStringArray>(s) {
                    Ok(ob) => {
                        debug!("Success");
                        Ok(ob)
                    },
                    Err(e) => {
                        debug!("Failed");
                        Err(e.to_string())
                    }
                }
            },
            Err(e) => {
                debug!("Failed");
                Err(e.to_string())
            }
        }
    }
}

impl<'t> DoRequest<response::Neighbors> for RequestBuilder<'t, response::Neighbors> {}
