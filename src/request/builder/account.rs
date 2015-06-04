use response;
use request::RequestBuilder;
use request::DoRequest;
use request::ActionRequest;
use request::ActionsRequest;

/// A type of `RequestBuilder` that lets you make requests for account related information
///
/// # Example
///
/// ```no_run
/// # use doapi::DoManager;
/// # use doapi::DoRequest;
/// let domgr = DoManager::with_token("<token>");
/// let account_request = domgr.account();
/// ```
pub type AccountRequest<'t> = RequestBuilder<'t, response::Account>;

impl<'t> AccountRequest<'t> {
    /// Returns a `RequestBuilder` for making requests about a single `response::Action`. 
    ///
    /// **Parameters:**
    /// `id`: The action ID you'd like to retrieve from DigitalOcean
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// # let domgr = DoManager::with_token("asfasdfasdf");
    /// // ... domgr set up same as before
    /// match domgr.account().action("1234").retrieve() {
    ///     Ok(action) => println!("Action: {}", action),
    ///     Err(e)     => println!("Error: {}", e)
    /// }
    /// ```
    pub fn action(self, id: &str) -> ActionRequest<'t> {
        // https://api.digitalocean.com/v2/actions/$ID
        RequestBuilder::new(self.auth, format!("https://api.digitalocean.com/v2/actions/{}", id))
    }

    /// Returns a `RequestBuilder` for making requests about multiple `response::Action`s. 
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use doapi::DoManager;
    /// # use doapi::DoRequest;
    /// # let domgr = DoManager::with_token("asfasdfasdf");
    /// // ... domgr set up same as before
    /// match domgr.account().actions().retrieve() {
    ///     Ok(action_vec) => println!("Action: {:?}", action_vec),
    ///     Err(e)     => println!("Error: {}", e)
    /// }
    /// ```
    pub fn actions(self) -> ActionsRequest<'t> {
        // https://api.digitalocean.com/v2/actions
        RequestBuilder::new(self.auth, "https://api.digitalocean.com/v2/actions")
    }
}

impl<'t> DoRequest<response::Account> for AccountRequest<'t> {}
