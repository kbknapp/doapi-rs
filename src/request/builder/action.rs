use response;
use request::RequestBuilder;
use request::DoRequest;

/// A type of `RequestBuilder` which allows you make requests for information related to a single
/// action
///
/// # Example
///
/// ```no_run
/// # use doapi::DoManager;
/// # use doapi::DoRequest;
/// let domgr = DoManager::with_token("<token>");
/// let action_request = domgr.account().action("<action id>");
/// ```
pub type ActionRequest<'t> = RequestBuilder<'t, response::Action>;

/// A type of `RequestBuilder` that lets you make requests for multiple actions or the concept of
/// "Actions" as a whole
///
/// # Example
///
/// ```no_run
/// # use doapi::DoManager;
/// # use doapi::DoRequest;
/// let domgr = DoManager::with_token("<token>");
/// let actions_request = domgr.account().actions();
/// ```
pub type ActionsRequest<'t> = RequestBuilder<'t, response::Actions>;

impl<'t> DoRequest<response::Action> for ActionRequest<'t> {}
