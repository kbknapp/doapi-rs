use crate::request::{DoRequest, RequestBuilder};
use crate::response;

impl<'a, 't> DoRequest<response::Action> for RequestBuilder<'a, 't, response::Action> {}
